use std::future::Future;
use std::time::Duration;
use crate::client::GameClient;
use crate::GameServer;

async fn run_single_client_test<F, Fut>(test_fn: F)
where
    F: FnOnce(GameClient) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let server = GameServer::run().await.unwrap();
    let server_address = *server.get_address();
    assert_eq!(server.get_connections_count().await.unwrap(), 0);

    let client_offloaded_task = tokio::task::spawn(async move {
        let client = GameClient::connect(server_address).await.unwrap();

        tracing::info!("Starting client-server test space");
        test_fn(client);

        tokio::time::sleep(Duration::from_millis(10)).await;
    });

    tokio::time::sleep(Duration::from_millis(10)).await;
    // server_handler.await_any_connection().await;
    // assert_eq!(server_handler.connections_count(), 1, "Client not connected");

    client_offloaded_task.await.unwrap();

    tokio::time::sleep(Duration::from_millis(10)).await;
    // server_handler.await_all_disconnect().await;
    // assert_eq!(server_handler.connections_count(), 0, "Client not disconnected");

    server.shutdown_gracefully().await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::testing::tests_trace_setup;
    use super::*;
    #[tokio::test]
    async fn test_client_server_connection() {
        tests_trace_setup();

        run_single_client_test(|client| async {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }).await;
    }
}