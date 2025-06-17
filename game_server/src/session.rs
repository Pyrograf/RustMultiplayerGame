use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct ConnectionSession {
    connection_id: u64,
    address: SocketAddr,
    session_task: JoinHandle<()>,
}

impl ConnectionSession {
    pub async fn new(connection_id: u64, stream: TcpStream, address: SocketAddr) -> Self {
        tracing::info!("Creating connection session for {:?}", address);
        let session_task = tokio::spawn(async move {
            tracing::info!("Entered connection session task");
            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        Self { connection_id, address, session_task }
    }
}
