use crate::session::ConnectionSession;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot, Notify};
use tokio::task::JoinHandle;
use crate::game::Game;

pub mod client;
pub mod session;
pub mod requests;
pub mod responses;
mod testing;
mod game;

#[derive(Debug, thiserror::Error)]
pub enum GameServerError {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error(transparent)]
    OneshotRecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type GameServerResult<T> = Result<T, GameServerError>;

pub struct GameServer {
    task_handle: JoinHandle<()>,
    local_address: SocketAddr,
    commands_tx: mpsc::Sender<ServerCommand>,
    connection_notifications: Arc<Notify>,
}

#[derive(Debug)]
pub enum ServerCommand {
    Shutdown,
    CountConnections(oneshot::Sender<usize>),
}

impl GameServer {
    const COMMANDS_QUEUE_SIZE: usize = 32;
    const SESSION_END_QUEUE_SIZE: usize = 16;

    pub async fn run() -> tokio::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let local_address = listener.local_addr()?;
        let (commands_tx, mut commands_rx) =
            mpsc::channel::<ServerCommand>(Self::COMMANDS_QUEUE_SIZE);

        let connection_notifications =  Arc::new(Notify::new());
        let connection_notifications_shared = connection_notifications.clone();

        let task_handle = tokio::task::spawn(async move {
            let mut next_connection_id = 0;
            let mut connection_sessions: Vec<ConnectionSession> = Vec::new();
            let (session_end_tx, mut session_end_rx) = mpsc::channel(Self::SESSION_END_QUEUE_SIZE);
            let game = Arc::new(Game::new().await);
            
            loop {
                tokio::select! {
                    incomming_connection = listener.accept() => {
                        connection_notifications_shared.notify_waiters();

                        if let Ok((stream, address)) = incomming_connection {
                            let new_connection_session = ConnectionSession::new(
                                next_connection_id,
                                stream,
                                address,
                                session_end_tx.clone(),
                                game.clone()
                            ).await;

                            connection_sessions.push(new_connection_session);
                            next_connection_id += 1;
                        } else {
                            tracing::warn!("connection immediately terminated");
                        }
                    },
                    dced_session_id = session_end_rx.recv() => {
                        // `None` will happen only if this task get dropped - dont care
                        if let Some(dced_session_id) = dced_session_id {
                            // Remove from stored session
                            // TODO better approach & container
                            match connection_sessions.iter().position(|conn| conn.get_id() == dced_session_id) {
                                Some(session_index) => {
                                    let _ = connection_sessions.remove(session_index);
                                },
                                None => { tracing::warn!("could not found disconnected session. Ignores");}
                            }
                        }
                    },
                    cmd = commands_rx.recv() => {
                        let cmd = cmd.unwrap_or(ServerCommand::Shutdown);
                        tracing::debug!("Commands received '{cmd:?}'");
                        match cmd {
                            ServerCommand::Shutdown => {
                                break;
                            },
                            ServerCommand::CountConnections(sender) => {
                                if let Err(_) = sender.send(connection_sessions.len()) {
                                    tracing::error!("Receiver closed before getting response");
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(Self {
            task_handle,
            local_address,
            commands_tx,
            connection_notifications,
        })
    }

    pub async fn shutdown_gracefully(self) -> std::io::Result<()> {
        tracing::info!("Gracefully shutting down...");
        if self
            .commands_tx
            .send(ServerCommand::Shutdown)
            .await
            .is_err()
        {
            let err_msg = "Could not sent shutdown signal";
            tracing::warn!(err_msg);
            return Err(std::io::Error::new(ErrorKind::Other, err_msg));
        }

        let _ = self.task_handle.await?;
        tracing::info!("Server got shutdown!");
        Ok(())
    }

    pub async fn await_shutdown(self) -> std::io::Result<()> {
        let _ = self.task_handle.await?;
        tracing::info!("Server got shutdown!");
        Ok(())
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.local_address
    }

    pub async fn get_connections_count(&self) -> GameServerResult<usize> {
        let (commands_tx, mut commands_rx) = oneshot::channel::<usize>();
        let cmd = ServerCommand::CountConnections(commands_tx);
        self.commands_tx
            .send(cmd)
            .await
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;
        Ok(commands_rx.await?)
    }

    pub async fn await_any_connection(&self) -> GameServerResult<usize> {
        loop {
            let count = self.get_connections_count().await?;
            if count > 0 {
                return Ok(count);
            }

            tokio::select! {
                _ = self.connection_notifications.notified() => {},
                _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {}
            }
        }
    }

    pub async fn await_all_disconnect(&self) -> GameServerResult<()> {
        loop {
            let count = self.get_connections_count().await?;
            if count == 0 {
                return Ok(());
            }

            tokio::select! {
                _ = self.connection_notifications.notified() => {},
                _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::tests_trace_setup;

    #[tokio::test]
    async fn test_starting_shutting_server() {
        tests_trace_setup();

        let server = crate::GameServer::run().await.unwrap();
        let server_address = server.get_address();
        tracing::debug!("server address: {}", server_address);
        let connections_count =  server.get_connections_count().await.unwrap();
        assert_eq!(connections_count, 0);
        server.shutdown_gracefully().await.unwrap();
    }
}
