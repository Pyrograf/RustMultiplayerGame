#[cfg(test)]
mod tests {
    use crate::testing::tests_trace_setup;
    use accounts_manager::client::AccountsManagerClient;
    use accounts_manager::AccountsManagerServer;
    use database_adapter::test::DatabaseTestAdapter;
    use database_adapter::AccountData;
    use game_server::client::GameClient;
    use game_server::GameServer;
    use std::sync::Arc;
    use database_adapter::character::CharacterId;

    #[tokio::test]
    async fn test_account_creation_character_creation_entering_game() {
        tests_trace_setup();

        let username = "ZenonsAccount";
        let password_plaintext = "osompassword";
        let character_name = "Janusz123";

        let (accounts_server, game_server) = {
            // Database, moved to not be used directly
            let database_adapter = Arc::new(DatabaseTestAdapter::new().await);

            (
                AccountsManagerServer::run(database_adapter.clone())
                    .await
                    .unwrap(),
                GameServer::run(database_adapter).await.unwrap(),
            )
        };

        // Accounts related server & client
        let accounts_manager_server_address = *accounts_server.get_address();
        let accounts_manager_client =
            AccountsManagerClient::new(&accounts_manager_server_address.to_string()).unwrap();
        let status = accounts_manager_client.get_server_status().await.unwrap();
        println!("No accounts so far: {:?}", status);

        // In-game (characters) related server & client
        let game_server_address = *game_server.get_address();
        let game_client = GameClient::connect(game_server_address).await.unwrap();

        // Create account
        accounts_manager_client.request_create_account(username.to_string(), password_plaintext.to_string()).await.unwrap();
        let status = accounts_manager_client.get_server_status().await.unwrap();
        println!("Account should be created: {:?}", status);

        // Create new character
        let new_character_id = accounts_manager_client.request_create_character(
            username.to_string(),
            password_plaintext.to_string(),
            character_name.to_string()
        ).await.unwrap();
        println!("New character ID: {:?}", new_character_id);

        // Count character of the account
        let characters_list = accounts_manager_client.request_account_characters(
            username.to_string(),
            password_plaintext.to_string(),
        ).await.unwrap();
        assert_eq!(characters_list.len(), 1);

        // Enter game world
        assert_eq!(game_client.get_entities_count().await.unwrap(), 0);

        game_client.attach_to_character(new_character_id).await.unwrap();
        assert_eq!(game_client.get_entities_count().await.unwrap(), 1);
    }
}
