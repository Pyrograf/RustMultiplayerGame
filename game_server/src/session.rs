use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use crate::{GameServerError, GameServerResult};
use crate::game::Game;
use crate::requests::GameServerRequest;
use crate::responses::GameServerResponse;

#[derive(Debug)]
pub struct ConnectionSession {
    connection_id: ConnectionSessionId,
    address: SocketAddr,
    session_task: JoinHandle<()>,
}

pub type ConnectionSessionId = u64;

impl ConnectionSession {
    pub async fn new(
        connection_id: ConnectionSessionId,
        mut stream: TcpStream,
        address: SocketAddr,
        disconnect_tx: mpsc::Sender<ConnectionSessionId>,
        game: Arc<Game>
    ) -> Self {
        tracing::info!("Creating connection session for {:?}", address);
        let session_task = tokio::spawn(async move {
            tracing::info!("Entered connection session task");
            loop {
                // tokio::time::sleep(Duration::from_millis(100)).await;
                match stream.read_u32_le().await {
                    Ok(request_length) => {
                        let mut request_buffer = vec![0u8; request_length as usize];
                        stream.read_exact(&mut request_buffer).await.unwrap();

                        match Self::process_request_into_response(request_buffer, game.clone()).await {
                            Ok(response_bytes) => {
                                let response_bytes_count = response_bytes.len() as u32;
                                stream.write_u32_le(response_bytes_count).await.unwrap();
                                stream.write_all(response_bytes.as_slice()).await.unwrap();
                            },
                            Err(e) => {
                                tracing::error!("Could not response, reason: '{e}'");
                            }
                        }

                    },
                    Err(_) => {
                        if disconnect_tx.send(connection_id).await.is_err() {
                            tracing::warn!("Could not inform about session end. Noone cares :(");
                        }
                        break;
                    }
                }
            }
        });

        Self { connection_id, address, session_task }
    }

    async fn process_request_into_response(response_buffer: Vec<u8>, game: Arc<Game>) -> GameServerResult<Vec<u8>> {
        let request: GameServerRequest = serde_json::from_slice(&response_buffer)
            .inspect_err(|e| tracing::error!("Error deserializing request: '{e}'"))?;

        let response = match request {
            GameServerRequest::Status => Self::handle_request_status(),
            GameServerRequest::EntitiesCount => Self::handle_request_entities_count(game).await,
        };

        let result_bytes = serde_json::to_vec(&response).inspect_err(|err| {
            tracing::error!("Error serializing request {request:?}. {:?}", err)
        }).inspect_err(|e| tracing::error!("Error serializing response: '{e}'"))?;

        Ok(result_bytes)
    }

    fn handle_request_status() -> GameServerResponse {
        GameServerResponse::Status {
            info: "Hello from session".to_string()
        }
    }

    async fn handle_request_entities_count(game: Arc<Game>) ->  GameServerResponse {
        GameServerResponse::EntitiesCount {
            count: game.world_manager.get_entities_count().await
        }
    }

    pub fn get_id(&self) -> ConnectionSessionId { self.connection_id }
}
