#[cfg(test)]
mod tests {
    use crate::client::{AccountsManagerClient, AccountsManagerClientError};
    use crate::testing::tests_trace_setup;
    use crate::AccountsManagerServer;
    use std::future::Future;
    use crate::account::AccountError;
    use crate::responses::ApiError;

    #[tokio::test]
    async fn test_client_connecting_to_server() {
        tests_trace_setup();

        let server = AccountsManagerServer::run().await.unwrap();
        let server_address = server.get_address().to_string();
        let client = AccountsManagerClient::new(&server_address).unwrap();
        let response = client.get_server_status().await.unwrap();
        assert!(!response.motd.is_empty());
    }

    async fn setup_server_client_interaction<P, Fut>(procedure: P)
    where
        P: FnOnce(AccountsManagerClient) -> Fut,
        Fut: Future<Output = ()>,
    {
        let server = AccountsManagerServer::run().await.unwrap();
        let server_address = server.get_address().to_string();
        let client = AccountsManagerClient::new(&server_address).unwrap();
        procedure(client).await
    }

    #[tokio::test]
    async fn test_creating_account() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            // First account created
            client.request_create_account("User1".to_string(), "Password1234%^&".to_string())
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);
        })
        .await;
    }

    #[tokio::test]
    async fn test_creating_account_should_not_create_already_existing_account() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            // First account created
            client.request_create_account("User1".to_string(), "Password1234%^&".to_string())
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);

            // Second account - not created, user already exists
            let response = client.request_create_account("User1".to_string(), "Password1234%^&".to_string())
                .await
                .unwrap_err();

            match response {
                AccountsManagerClientError::ApiError(api_err) => match api_err {
                    ApiError::AccountError(acc_err) => match acc_err {
                        AccountError::UsernameAlreadyExists =>  (),
                        err => panic!("Unexpected error: {:?} should give 'AccountError::UsernameAlreadyExists'", err),
                    }
                },
                _ => panic!("Should give response 'ApiError', got instead {response:?}"),
            }

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1); // Count remains the same
        }).await;
    }

    #[tokio::test]
    async fn test_deleting_account_with_correct_password() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            let username = "User1";
            let password = "Password1234%^&";

            // First account created
            client.request_create_account(username.to_string(), password.to_string())
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);

            // Delete account with correct password
            client.request_delete_account(username.to_string(), password.to_string())
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0); // Count decreased
        }).await;
    }

    #[tokio::test]
    async fn test_deleting_account_with_bad_password_should_fail() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            let username = "User1";

            // First account created
            client.request_create_account(username.to_string(), "Password1234%^&".to_string())
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);

            // Delete account with bad password
            let deletion_error = client.request_delete_account(username.to_string(), "Password1234%^".to_string())
                .await
                .unwrap_err();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1); // Count remains the same

            assert!(matches!(deletion_error, AccountsManagerClientError::ApiError(ApiError::AccountError(AccountError::BadPassword))));
            
        }).await;
    }

    #[tokio::test]
    async fn test_deleting_account_with_bad_username_should_fail() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            let username = "User5";

            // First account created
            client.request_create_account(username.to_string(), "Password1234%^&".to_string())
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);

            // Delete account with bad username
            let deletion_error = client.request_delete_account("Noname105".to_string(), "Password1234%^".to_string())
                .await
                .unwrap_err();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1); // Count remains the same

            assert!(matches!(deletion_error, AccountsManagerClientError::ApiError(ApiError::AccountError(AccountError::UsernameNotFound))));

        }).await;
    }
}
