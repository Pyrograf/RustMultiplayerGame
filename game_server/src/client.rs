use crate::requests::GameServerRequest;
use crate::responses::{GameServerResponse, ResponseResult};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use crate::game::character::CharacterId;

#[derive(Debug, thiserror::Error)]
pub enum GameClientError {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error(transparent)]
    OneshotRecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Bad response")]
    BadResponse,

    #[error("Other '{0}'")]
    Other(String),
}

pub type GameClientResult<T> = Result<T, GameClientError>;

pub struct GameClientRequest {
    content: GameServerRequest,
    response_tx: oneshot::Sender<GameServerResponse>,
}

impl GameClientRequest {
    pub fn wrap(request: GameServerRequest) -> (Self, oneshot::Receiver<GameServerResponse>) {
        let (response_tx, response_rx) = oneshot::channel();
        (
            Self {
                content: request,
                response_tx,
            },
            response_rx,
        )
    }
}

pub struct GameClient {
    requests_tx: mpsc::Sender<GameClientRequest>,
    task: JoinHandle<()>,
}

impl GameClient {
    pub async fn connect<A: ToSocketAddrs + Debug>(addr: A) -> GameClientResult<Self> {
        tracing::info!("Client attempts to connect to server {addr:?}...");

        let (requests_tx, mut requests_rx) = mpsc::channel::<GameClientRequest>(1);

        let mut stream = TcpStream::connect(addr).await?;

        let task = tokio::task::spawn(async move {
            loop {
                // Process events
                tokio::select! {
                    request = requests_rx.recv() => {
                        match request {
                            Some(request) => {
                                // Send request await response
                                match Self::process_request(&mut stream, request.content).await {
                                    Ok(response) => {
                                        if request.response_tx.send(response).is_err() {
                                            tracing::warn!("Response channel closed.");
                                        }
                                    },
                                    Err(e) => {
                                        tracing::error!("Has no response, droping channel");
                                        // oneshot will be shut soon at drop
                                    }
                                }
                            },
                            None => {
                                tracing::info!("Client is getting shutdown. Disconnect soon...");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(Self { requests_tx, task })
    }

    async fn process_request(
        stream: &mut TcpStream,
        request: GameServerRequest,
    ) -> GameClientResult<GameServerResponse> {
        let request_bytes = serde_json::to_vec(&request).inspect_err(|err| {
            tracing::error!("Error serializing request {request:?}. {:?}", err)
        })?;

        let length = request_bytes.len() as u32;

        stream.write_u32_le(length).await?;
        stream.write_all(request_bytes.as_slice()).await?;

        let response_length = stream.read_u32_le().await?;
        tracing::info!("Client sent request, awaiting response size: {response_length}...");
        let mut response_buff = vec![0u8; response_length as usize];
        stream.read_exact(&mut response_buff).await.inspect_err(|e| {
            tracing::error!("Error reading from response stream: '{e:?}' exact size: {response_length}")
        })?;

        Ok(serde_json::from_slice(&response_buff)
            .inspect_err(|e| tracing::error!("Error deserializing response: '{e}'"))?
        )
    }

    pub async fn make_request(
        &self,
        request: GameServerRequest,
    ) -> GameClientResult<GameServerResponse> {
        let (request, response_rx) = GameClientRequest::wrap(request);
        self.requests_tx
            .send(request)
            .await
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;
        Ok(response_rx.await?)
    }

    pub async fn attach_to_character(&self, character_id: CharacterId) -> GameClientResult<()> {
        let response = self.make_request(GameServerRequest::AttachToCharacter { character_id }).await?;
        match response {
            GameServerResponse::AttachToCharacter { result } => match result {
                ResponseResult::Success => Ok(()),
                ResponseResult::Error { message } =>  Err(GameClientError::Other(message)),
            },
            _ => Err(GameClientError::BadResponse)
        }
    }

    pub async fn get_entities_count(&self) -> GameClientResult<usize> {
        let response = self.make_request(GameServerRequest::EntitiesCount).await?;
        match response {
            GameServerResponse::EntitiesCount { count } => Ok(count),
            _ => Err(GameClientError::BadResponse)
        }
    }

    pub async fn get_status(&self) -> GameClientResult<String> {
        let response = self.make_request(GameServerRequest::Status).await?;
        match response {
            GameServerResponse::Status { info } => Ok(info),
            _ => Err(GameClientError::BadResponse)
        }
    }

    pub async fn disconnect_await_finished(self) {
        drop(self.requests_tx);
        let _ = self.task.await.expect("Finishing client's task failed");
    }
}
