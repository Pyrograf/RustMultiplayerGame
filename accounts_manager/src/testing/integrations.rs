#[cfg(test)]
mod tests {
    use crate::AccountsManagerServer;
    use crate::client::AccountsManagerClient;
    use crate::testing::tests_trace_setup;

    #[tokio::test]
    async fn test_client_connecting_to_server() {
        tests_trace_setup();

        let server = AccountsManagerServer::run().await.unwrap();
        let server_address = server.get_address().to_string();
        let client = AccountsManagerClient::new(&server_address).unwrap();
        let response = client.get_server_status().await.unwrap();
        assert!(!response.motd.is_empty());
    }

    #[tokio::test]
    async fn test_creating_account() {
        tests_trace_setup();

        let server = AccountsManagerServer::run().await.unwrap();
        let server_address = server.get_address().to_string();
        let client = AccountsManagerClient::new(&server_address).unwrap();

        // TODO accounts count 0
        let response = client.request_create_account("User1".to_string(), "Password1234%^&".to_string()).await.unwrap();
        tracing::info!("1: {:?}", response);
        let response = client.request_create_account("User1".to_string(), "Password1234%^&".to_string()).await.unwrap_err();
        tracing::info!("2: {:?}", response);
        // TODO accounts count 1
    }
}
