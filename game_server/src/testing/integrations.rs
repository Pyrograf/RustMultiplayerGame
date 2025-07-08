#[cfg(test)]
mod tests {
    use crate::requests::GameServerRequest;
    use crate::responses::GameServerResponse;
    use crate::testing::tests_trace_setup;
    use std::future::Future;
    use std::sync::Arc;
    use std::time::Duration;
    use database_adapter::test::DatabaseTestAdapter;
    use crate::client::GameClient;
    use crate::GameServer;

    fn run_single_client_test<F, Fut>(test_fn: F)
    where
        F: FnOnce(GameClient) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let database_adapter = Arc::new(DatabaseTestAdapter::with_test_data().await);
            let server = GameServer::run(database_adapter).await.unwrap();
            let server_address = *server.get_address();
            assert_eq!(server.get_connections_count().await.unwrap(), 0);

            let client_offloaded_task = tokio::task::spawn(async move {
                let client = GameClient::connect(server_address).await.unwrap();

                tracing::info!("Starting client-server test space");
                test_fn(client).await;
            });

            client_offloaded_task.await.unwrap();

            // Here client should be disconnected
            server.await_all_disconnect().await.unwrap();
            assert_eq!(server.get_connections_count().await.unwrap(), 0, "Client not disconnected");

            server.shutdown_gracefully().await.unwrap();
        });
    }

    #[test]
    fn test_client_server_connection() {
        tests_trace_setup();

        run_single_client_test(|_client| async {
            let span = tracing::debug_span!("test_client_server_connection");
            let _guard = span.enter();

            tokio::time::sleep(Duration::from_millis(1)).await;
        });
    }

    #[test]
    fn test_client_getting_server_status() {
        tests_trace_setup();

        run_single_client_test(|client| async move {
            let span = tracing::debug_span!("test_client_getting_server_status");
            let _guard = span.enter();

            let response = client.make_request(GameServerRequest::Status).await.unwrap();
            match response {
                GameServerResponse::Status { info } => { tracing::info!("Got status response: {}", info); },
                _ => panic!("Got unexpected response: {response:?}"),
            }
        });
    }

    #[test]
    fn test_client_getting_entities_count() {
        tests_trace_setup();

        run_single_client_test(|client| async move {
            let span = tracing::debug_span!("test_client_getting_entities_count");
            let _guard = span.enter();

            let response = client.make_request(GameServerRequest::EntitiesCount).await.unwrap();
            match response {
                GameServerResponse::EntitiesCount { count } => { assert_eq!(count, 0, "Count not zero") ; },
                _ => panic!("Got unexpected response: {response:?}"),
            }
        });
    }

    #[test]
    fn test_client_attaching_to_character() {
        tests_trace_setup();

        run_single_client_test(|client| async move {
            let span = tracing::debug_span!("test_client_getting_entities_count");
            let _guard = span.enter();

            let entities_count = client.get_entities_count().await.unwrap();
            assert_eq!(entities_count, 0);

            client.attach_to_character(1).await.unwrap();

            let entities_count = client.get_entities_count().await.unwrap();
            assert_eq!(entities_count, 1);
        });
    }
}
