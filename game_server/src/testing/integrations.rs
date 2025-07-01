#[cfg(test)]
mod tests {
    use crate::requests::GameServerRequest;
    use crate::responses::GameServerResponse;
    use crate::testing::tests_trace_setup;
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
}






//////////////////////////////////////////

// fn run_single_client_test<F, Fut>(test_fn: F)
// where
//     F: FnOnce(GameClient) -> Fut + Send + 'static,
//     Fut: Future<Output = ()> + Send + 'static,
// {
//     let barrier = Arc::new(Barrier::new(2));
//     let barrier_shared = barrier.clone();
//
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async move {
//         let server = GameServer::run().await.unwrap();
//         let server_address = *server.get_address();
//         assert_eq!(server.get_connections_count().await.unwrap(), 0);
//
//         let client_offloaded_task = tokio::task::spawn(async move {
//             tracing::info!("Client awaits starting server...");
//             barrier_shared.wait().await;
//             // tokio::time::sleep(Duration::from_millis(1000)).await;
//
//             let client = GameClient::connect(server_address).await.unwrap();
//
//             tracing::info!("Starting client-server test space");
//             test_fn(client).await;
//         });
//
//         barrier.wait().await;
//         let connections_count = server.await_any_connection().await.unwrap();
//         assert_eq!(connections_count, 1, "Client not connected");
//
//         client_offloaded_task.await.unwrap();
//
//         server.await_all_disconnect().await.unwrap();
//         assert_eq!(server.get_connections_count().await.unwrap(), 0, "Client not disconnected");
//
//         server.shutdown_gracefully().await.unwrap();
//     });
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::requests::GameServerRequest;
//     use crate::responses::GameServerResponse;
//     use crate::testing::tests_trace_setup;
//     use super::*;
//
//     #[test]
//     fn test_client_server_connection() {
//         tests_trace_setup();
//
//         let span = tracing::error_span!("test_client_server_connection");
//         let _guard = span.enter();
//
//         run_single_client_test(|_client| async {
//             tokio::time::sleep(Duration::from_millis(1)).await;
//         });
//     }
//
//     #[test]
//     fn test_client_getting_server_status() {
//         tests_trace_setup();
//
//         let span = tracing::error_span!("test_client_getting_server_status");
//         let _guard = span.enter();
//
//         run_single_client_test(|client| async move {
//             let response = client.make_request(GameServerRequest::Status).await.unwrap();
//             match response {
//                 GameServerResponse::Status { info } => { tracing::info!("Got status response: {}", info); },
//                 _ => panic!("Got unexpected response: {response:?}"),
//             }
//
//             tokio::time::sleep(Duration::from_millis(100)).await;
//         });
//     }
//
//     #[test]
//     fn test_client_getting_entities_count() {
//         tests_trace_setup();
//
//         let span = tracing::error_span!("test_client_getting_entities_count");
//         let _guard = span.enter();
//         run_single_client_test(|client| async move {
//             let response = client.make_request(GameServerRequest::EntitiesCount).await.unwrap();
//             match response {
//                 GameServerResponse::EntitiesCount { count } => { assert_eq!(count, 0, "Count not zero") ; },
//                 _ => panic!("Got unexpected response: {response:?}"),
//             }
//
//             tokio::time::sleep(Duration::from_millis(100)).await;
//         });
//     }
// }


//////////////////////////////////////////////////










// async fn run_single_client_test<F, Fut>(test_fn: F)
// where
//     F: FnOnce(GameClient) -> Fut + Send + 'static,
//     Fut: Future<Output = ()> + Send + 'static,
// {
//     let server = GameServer::run().await.unwrap();
//     let server_address = *server.get_address();
//     assert_eq!(server.get_connections_count().await.unwrap(), 0);
//
//     tokio::time::sleep(Duration::from_millis(10)).await;
//
//     let client_offloaded_task = tokio::task::spawn(async move {
//         // Delay to enforce order, funky client connection can be faster
//         // than polling for connections count
//         tokio::time::sleep(Duration::from_millis(100)).await;
//
//         tracing::warn!("in task: Client connecting");
//         let client = GameClient::connect(server_address).await.unwrap();
//
//         tracing::info!("Starting client-server test space");
//         test_fn(client).await;
//
//         tokio::time::sleep(Duration::from_millis(100)).await;
//     });
//
//     tracing::warn!("1");
//     let connections_count = server.await_any_connection().await.unwrap();
//     assert_eq!(connections_count, 1, "Client not connected");
//
//     tracing::warn!("2");
//     client_offloaded_task.await.unwrap();
//
//     tracing::warn!("3");
//     server.await_all_disconnect().await.unwrap();
//     assert_eq!(server.get_connections_count().await.unwrap(), 0, "Client not disconnected");
//
//     tracing::warn!("4");
//     server.shutdown_gracefully().await.unwrap();
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::requests::GameServerRequest;
//     use crate::responses::GameServerResponse;
//     use crate::testing::tests_trace_setup;
//     use super::*;
//
//     #[tokio::test]
//     async fn test_client_server_connection() {
//         tests_trace_setup();
//
//         let span = tracing::error_span!("test_client_server_connection");
//         let _guard = span.enter();
//         run_single_client_test(|_client| async {
//             tokio::time::sleep(Duration::from_millis(10)).await;
//         }).await;
//     }
//
//     #[tokio::test]
//     async fn test_client_getting_server_status() {
//         tests_trace_setup();
//
//         let span = tracing::error_span!("test_client_getting_server_status");
//         let _guard = span.enter();
//         run_single_client_test(|client| async move {
//             let response = client.make_request(GameServerRequest::Status).await.unwrap();
//             match response {
//                 GameServerResponse::Status { info } => { tracing::info!("Got status response: {}", info); },
//                 _ => panic!("Got unexpected response: {response:?}"),
//             }
//
//             tokio::time::sleep(Duration::from_millis(100)).await;
//         }).await;
//     }
//
//     // #[tokio::test]
//     // async fn test_client_getting_entities_count() {
//     //     tests_trace_setup();
//     //
//     //     run_single_client_test(|client| async move {
//     //         let response = client.make_request(GameServerRequest::EntitiesCount).await.unwrap();
//     //         match response {
//     //             GameServerResponse::EntitiesCount { count } => { assert_eq!(count, 0, "Count not zero") ; },
//     //             _ => panic!("Got unexpected response: {response:?}"),
//     //         }
//     //
//     //         tokio::time::sleep(Duration::from_millis(100)).await;
//     //     }).await;
//     // }
// }