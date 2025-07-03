use std::io::ErrorKind;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::app_data::AppData;
use tower_http::trace::{
    DefaultMakeSpan,
    DefaultOnRequest,
    DefaultOnResponse,
    TraceLayer
};
use database_adapter::DatabaseAdapter;

pub mod router;
pub mod app_data;
pub mod handlers;
pub mod client;
pub mod requests;
pub mod responses;
pub mod services;
mod testing;

#[derive(Debug)]
pub struct AccountsManagerServer {
    task_handle: tokio::task::JoinHandle<Result<(), std::io::Error>>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    address: std::net::SocketAddr,
    app_data: Arc<Mutex<AppData>>,
}

impl AccountsManagerServer {
    pub async fn run(database_adapter: Arc<dyn DatabaseAdapter>) -> tokio::io::Result<Self> {

        let app_data = Arc::new(Mutex::new(AppData::new(database_adapter)));

        let app = router::get_router(app_data.clone())
            .layer(TraceLayer::new_for_http()
                       .make_span_with(DefaultMakeSpan::new().include_headers(true))
                       .on_request(DefaultOnRequest::new().level(tracing::Level::DEBUG))
                       .on_response(DefaultOnResponse::new().level(tracing::Level::DEBUG)),
            );

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;

        tracing::info!("{}({}) listening on {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), address);

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

        let task_handle = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    shutdown_rx.await.ok();
                })
                .await
        });

        Ok(
            Self {
                task_handle,
                shutdown_tx: Some(shutdown_tx),
                address,
                app_data,
            }
        )
    }

    pub fn shutdown_gracefully(&mut self) -> Result<(), std::io::Error> {
        tracing::info!("Gracefully shutting down...");

        if let Some(sender) = self.shutdown_tx.take() {
            if sender.send(()).is_err() {
                let err_msg = "Could not sent shutdown signal";
                tracing::warn!(err_msg);
                return Err(std::io::Error::new(ErrorKind::Other, err_msg));
            }
        } else {
            let err_msg = "Shutdown send was already used";
            tracing::warn!(err_msg);
            return Err(std::io::Error::new(ErrorKind::Other, err_msg));
        }

        Ok(())
    }

    pub async fn await_shutdown(self) -> Result<(), std::io::Error> {
        let _ = self.task_handle.await?;
        tracing::info!("Server got shutdown!");
        Ok(())
    }

    pub async fn shutdown_gracefully_await(mut self) -> Result<(), std::io::Error> {
        self.shutdown_gracefully()?;
        self.await_shutdown().await
    }

    pub fn get_address(&self) -> &std::net::SocketAddr {
        &self.address
    }

    pub fn get_url(&self) -> String {
        format!("http://{}", self.address)
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use database_adapter::test::DatabaseTestAdapter;
    use crate::testing::tests_trace_setup;
    use super::*;

    #[tokio::test]
    async fn test_dropping_server_to_gracefully_shutdown() {
        tests_trace_setup();

        {
            let db = DatabaseTestAdapter::new().await;
            let server = AccountsManagerServer::run(Arc::new(db)).await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    #[tokio::test]
    async fn test_running_graceful_shutdown() {
        tests_trace_setup();

        let db = DatabaseTestAdapter::new().await;
        let server = AccountsManagerServer::run(Arc::new(db)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        server.shutdown_gracefully_await().await.unwrap();
    }
}
