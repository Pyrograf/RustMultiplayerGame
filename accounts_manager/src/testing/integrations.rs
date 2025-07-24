#[cfg(test)]
mod tests {
    use crate::client::{AccountsManagerClient, AccountsManagerClientError};
    use crate::testing::tests_trace_setup;
    use crate::AccountsManagerServer;
    use std::future::Future;
    use std::sync::Arc;
    use database_adapter::DatabaseAdapterError;
    use database_adapter::test::DatabaseTestAdapter;
    use crate::responses::ApiError;

    #[tokio::test]
    async fn test_client_connecting_to_server() {
        tests_trace_setup();
        let database_adapter = Arc::new(DatabaseTestAdapter::new().await);

        let server = AccountsManagerServer::run(database_adapter).await.unwrap();
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
        let database_adapter = Arc::new(DatabaseTestAdapter::new().await);
        let server = AccountsManagerServer::run(database_adapter).await.unwrap();
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
    async fn test_login_get_account_details() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            let username = "User1".to_string();
            let password = "Password1234%^&".to_string();

            // First account created
            client.request_create_account(username.clone(), password.clone() ).await.unwrap();

            let token = client.request_login_to_account(username.clone(), password.clone()).await
                .unwrap();

            let _ = client.request_account_details(username.clone(), &token).await
                .unwrap();;

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);
        })
        .await;
    }

    #[tokio::test]
    async fn test_login_logout_account() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            let username = "User1".to_string();
            let password = "Password1234%^&".to_string();

            // First account created
            client.request_create_account(username.clone(), password.clone() ).await.unwrap();

            let token = client.request_login_to_account(username.clone(), password.clone()).await
                .unwrap();

            let _ = client.request_account_details(username.clone(), &token).await.unwrap();

            client.request_logout_account(username.clone(), token).await.unwrap();

            let request_details_err = client.request_account_details(username.clone(), &"notoken:(".to_string()).await;
            assert!(matches!(request_details_err, Err(AccountsManagerClientError::Unauthorized)));

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
                    ApiError::DatabaseAdapterError(acc_err) => match acc_err {
                        DatabaseAdapterError::UsernameAlreadyExists =>  (),
                        err => panic!("Unexpected error: {:?} should give 'DatabaseAdapterError::UsernameAlreadyExists'", err),
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

            let token = client.request_login_to_account(username.to_string(), password.to_string()).await
                .unwrap();

            // Delete account with correct password
            client.request_delete_account(username.to_string(), token)
                .await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0); // Count decreased
        }).await;
    }

    #[tokio::test]
    async fn test_deleting_account_with_bad_token_should_fail() {
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

            let _token = client.request_login_to_account(username.to_string(), password.to_string()).await
                .unwrap();

            // Delete account with bad password
            let deletion_error = client.request_delete_account(username.to_string(), "bad_token1234".to_string())
                .await
                .unwrap_err();
            assert!(matches!(deletion_error, AccountsManagerClientError::Unauthorized));

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1); // Count remains the same
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
            let password = "User5";

            // First account created
            client.request_create_account(username.to_string(), password.to_string())
                .await
                .unwrap();

            let token = client.request_login_to_account(username.to_string(), password.to_string()).await
                .unwrap();

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1);

            // Delete account with bad username
            let deletion_error = client.request_delete_account("Noname105".to_string(), token)
                .await
                .unwrap_err();
            assert!(matches!(deletion_error, AccountsManagerClientError::ApiError(ApiError::DatabaseAdapterError(DatabaseAdapterError::UsernameNotFound))));

            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 1); // Count remains the same
        }).await;
    }

    #[tokio::test]
    async fn test_update_account_password_with_correct_old_password() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            let username = "User5";
            let password_old = "Password1234%^&";
            client.request_create_account(username.to_string(), password_old.to_string())
                .await
                .unwrap();

            let token = client.request_login_to_account(username.to_string(), password_old.to_string())
                .await
                .unwrap();
            
            // Change password with correct old password
            let password_new = "Password1234%^&111111111";
            client.request_update_account_password(username.to_string(), password_old.to_string(), password_new.to_string(), &token)
                .await
                .unwrap();

            // Reuse old password - should fail
            let password_old_incorrect = password_old;
            
            let update_password_error = client.request_update_account_password(username.to_string(), password_old_incorrect.to_string(), "sometingdontcare".to_string(), &token)
                .await
                .unwrap_err();
            
            assert!(matches!(update_password_error, AccountsManagerClientError::ApiError(ApiError::DatabaseAdapterError(DatabaseAdapterError::BadPassword))));
        }).await;
    }


    #[tokio::test]
    async fn test_creating_character() {
        tests_trace_setup();

        setup_server_client_interaction(|client| async move {
            // Initially no accounts
            let response = client.get_server_status().await.unwrap();
            assert_eq!(response.accounts_count, 0);

            let username = "User1";
            let password = "Password1234%^&";
            let character_name = "Janusz";

            // First account created
            client.request_create_account(username.to_string(), password.to_string())
                .await
                .unwrap();

            // Login
            let token = client.request_login_to_account(username.to_string(), password.to_string()).await
                .unwrap();

            // Get all  - initially zero
            let characters_data = client.request_account_characters(username.to_string(), &token)
                .await
                .unwrap();
            assert_eq!(characters_data.len(), 0);

            // Create character
            let new_character_id = client.request_create_character(username.to_string(), character_name.to_string(), &token)
                .await
                .unwrap();

            // Get all characters
            let characters_data = client.request_account_characters(username.to_string(), &token)
                .await
                .unwrap();
            assert_eq!(characters_data.len(), 1);

            let the_one_character = characters_data.get(0).unwrap();
            assert_eq!(the_one_character.name, character_name);
            assert_eq!(the_one_character.id, new_character_id);
        }).await;
    }
}
