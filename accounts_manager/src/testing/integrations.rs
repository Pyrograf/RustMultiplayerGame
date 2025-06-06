#[cfg(test)]
mod tests {
    use crate::AccountsManagerServer;
    use crate::client::AccountsManagerClient;
    use crate::testing::tests_trace_setup;

    #[tokio::test]
    async fn text_client_connecting_to_server() {
        tests_trace_setup();

        let server = AccountsManagerServer::run().await.unwrap();
        let server_address = server.get_address().to_string();
        let client = AccountsManagerClient::new(&server_address).unwrap();
        let response = client.get_server_status().await.unwrap();
        assert!(!response.motd.is_empty());
    }
}
