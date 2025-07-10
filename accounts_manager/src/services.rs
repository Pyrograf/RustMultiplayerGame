use std::sync::Arc;
use database_adapter::{AccountData, DatabaseAdapter, DatabaseAdapterError, DatabaseAdapterResult};
use database_adapter::character::{CharacterId, NewCharacterData};

async fn verify_password(
    username: &str,
    password: &str,
    database_adapter: Arc<dyn DatabaseAdapter>,
) -> DatabaseAdapterResult<()> {
    let account = database_adapter
        .get_account_by_name(&username).await?;
    let password_matches = account.verify(&password)?;
    if !password_matches {
        return Err(DatabaseAdapterError::BadPassword);
    } else {
        Ok(())
    }
}

pub async fn create_account(
    username: String,
    password: String,
    database_adapter: Arc<dyn DatabaseAdapter>,
) -> DatabaseAdapterResult<()> {
    let new_account = AccountData::new(username, &password)?;
    database_adapter.add_account(new_account).await?;
    Ok(())
}

pub async fn delete_account(
    username: String,
    password: String,
    database_adapter: Arc<dyn DatabaseAdapter>,
) -> DatabaseAdapterResult<()> {
    verify_password(&username, &password, database_adapter.clone()).await?;
    database_adapter.remove_account_with_username(&username).await?;
    Ok(())
}

pub async fn update_account_password(
    username: String,
    password_old: String,
    password_new: String,
    database_adapter: Arc<dyn DatabaseAdapter>,
) -> DatabaseAdapterResult<()> {
    database_adapter.change_password(&username, &password_old, &password_new).await?;
    Ok(())
}

pub async fn get_characters_of_account(
    username: String,
    database_adapter: Arc<dyn DatabaseAdapter>,
) -> DatabaseAdapterResult<Vec<CharacterId>> {
    let characters = database_adapter.get_characters_of_account(&username).await?;
    Ok(characters)
}

pub async fn create_character_for_account(
    username: String,
    password: String,
    character_name: String,
    database_adapter: Arc<dyn DatabaseAdapter>,
) -> DatabaseAdapterResult<CharacterId> {
    // Here new character get additional initial data.
    // Consider if it is proper place.
    // For sure better than handler or request from client.
    let new_character = NewCharacterData {
        name: character_name,
        position_x: 0.0,
        position_y: 0.0,
        speed: 1.0,
    };

    let new_character_id = database_adapter.add_character(new_character).await?;
    database_adapter.attach_character_to_account(&username, new_character_id).await?;
    Ok(new_character_id)
}

#[cfg(test)]
mod tests {
    use database_adapter::DatabaseAdapterError;
    use database_adapter::test::DatabaseTestAdapter;
    use super::*;

    #[tokio::test]
    async fn test_creating_account() {
        let database_adapter = Arc::new(DatabaseTestAdapter::new().await);

        create_account(
            "User1".to_string(),
            "psswdrad123456%^&*".to_string(),
            database_adapter.clone(),
        ).await
        .unwrap();

        create_account(
            "User2".to_string(),
            "psswdrad123456%^&*".to_string(),
            database_adapter.clone(),
        ).await
        .unwrap();

        create_account(
            "User".to_string(),
            "psswdrad123456%^&*".to_string(),
            database_adapter.clone(),
        ).await
        .unwrap();

        let err_user_alredy_exists = create_account(
            "User1".to_string(),
            "psswdrad123456%^&*".to_string(),
            database_adapter.clone(),
        ).await
        .unwrap_err();

        assert!(
            matches!(err_user_alredy_exists, DatabaseAdapterError::UsernameAlreadyExists),
            "Err {:?}",
            err_user_alredy_exists
        );
        assert_eq!(database_adapter.get_accounts_count().await.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_delete_account() {
        let database_adapter = Arc::new(DatabaseTestAdapter::new().await);

        let account_1 = AccountData::new("User11".to_string(), "qwertyuio12345$%^&").unwrap();
        database_adapter.add_account(account_1).await.unwrap();
        assert_eq!(database_adapter.get_accounts_count().await.unwrap(), 1);

        let account_2_user = "User12";
        let account_2_psswrd = "mehmeh12345$%^&";
        let account_2 = AccountData::new(account_2_user.to_string(), account_2_psswrd).unwrap();
        database_adapter.add_account(account_2).await.unwrap();
        assert_eq!(database_adapter.get_accounts_count().await.unwrap(), 2);

        delete_account(
            account_2_user.to_string(),
            account_2_psswrd.to_string(),
            database_adapter.clone(),
        ).await
        .unwrap();
        assert_eq!(database_adapter.get_accounts_count().await.unwrap(), 1);

        let err_user_not_found = delete_account(
            account_2_user.to_string(),
            account_2_psswrd.to_string(),
            database_adapter.clone(),
        ).await
        .unwrap_err();

        assert!(
            matches!(err_user_not_found, DatabaseAdapterError::UsernameNotFound),
            "Err {:?}",
            err_user_not_found
        );
        assert_eq!(database_adapter.get_accounts_count().await.unwrap(), 1);
    }
}
