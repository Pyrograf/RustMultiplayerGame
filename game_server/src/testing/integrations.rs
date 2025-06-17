use std::future::Future;
use std::time::Duration;
use crate::client::GameClient;
use crate::GameServer;

fn run_single_client_test<F, Fut>(test_fn: F)
where
    F: FnOnce(GameClient) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let server = GameServer::run().await.unwrap();
        let server_address = *server.get_address();
        assert_eq!(server.get_connections_count().await.unwrap(), 0);


        let client_offloaded_task = tokio::task::spawn(async move {
            // Delay to enforce order, funky client connection can be faster
            // than polling for connections count
            tracing::info!("A");
            tokio::time::sleep(Duration::from_millis(100)).await;

            tracing::info!("B");
            let client = GameClient::connect(server_address).await.unwrap();

            tracing::info!("C");
            tracing::info!("Starting client-server test space");
            test_fn(client).await;
            tracing::info!("D");
        });

        tracing::info!("1");
        let connections_count = server.await_any_connection().await.unwrap();
        assert_eq!(connections_count, 1, "Client not connected");

        tracing::info!("2");
        client_offloaded_task.await.unwrap();

        tracing::info!("3");
        server.await_all_disconnect().await.unwrap();
        assert_eq!(server.get_connections_count().await.unwrap(), 0, "Client not disconnected");

        tracing::info!("4");
        server.shutdown_gracefully().await.unwrap();
    });
}

#[cfg(test)]
mod tests {
    use crate::requests::GameServerRequest;
    use crate::responses::GameServerResponse;
    use crate::testing::tests_trace_setup;
    use super::*;

    #[test]
    fn test_client_server_connection() {
        tests_trace_setup();

        run_single_client_test(|_client| async {
            tokio::time::sleep(Duration::from_millis(1)).await;
        });
    }

    #[test]
    fn test_client_getting_server_status() {
        tests_trace_setup();

        run_single_client_test(|client| async move {
            tracing::error!("X");
            let response = client.make_request(GameServerRequest::Status).await.unwrap();
            match response {
                GameServerResponse::Status { info } => { tracing::error!("Got status response: {}", info); }
            }

            tracing::error!("Y");
            tokio::time::sleep(Duration::from_millis(100)).await;
            tracing::error!("Z");
        });
    }
}