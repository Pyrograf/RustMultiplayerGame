use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct ConnectionSession {
    connection_id: ConnectionSessionId,
    address: SocketAddr,
    session_task: JoinHandle<()>,
}

pub type ConnectionSessionId = u64;

impl ConnectionSession {
    pub async fn new(connection_id: ConnectionSessionId, mut stream: TcpStream, address: SocketAddr, disconnect_tx: mpsc::Sender<ConnectionSessionId>) -> Self {
        tracing::info!("Creating connection session for {:?}", address);
        let session_task = tokio::spawn(async move {
            tracing::info!("Entered connection session task");
            loop {
                // tokio::time::sleep(Duration::from_millis(100)).await;
                match stream.read_u32_le().await {
                    Ok(request_length) => {
                        let mut buffer = vec![0u8; request_length as usize];
                        stream.read_exact(&mut buffer).await.unwrap();

                        stream.write_u32_le(2).await.unwrap();
                        stream.write_all("1234".as_bytes()).await.unwrap();

                        // TODO process requests
                        // unimplemented!("TODO")
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

    pub fn get_id(&self) -> ConnectionSessionId { self.connection_id }
}
