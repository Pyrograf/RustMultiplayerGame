use crate::{AccountData, DatabaseAdapter, DatabaseAdapterError, DatabaseAdapterResult};
use std::collections::HashSet;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::character::{CharacterData, CharacterId, NewCharacterData};

struct CharactersManager {
    pub characters: HashSet<CharacterData>,
    pub new_character_id: CharacterId,
}

impl CharactersManager {
    pub fn new() -> Self {
        Self {
            characters: HashSet::new(),
            new_character_id: 0,
        }
    }
}

pub struct DatabaseTestAdapter {
    accounts: Mutex<HashSet<AccountData>>,
    characters_manager: Mutex<CharactersManager>,
}

#[async_trait]
impl DatabaseAdapter for DatabaseTestAdapter {
    async fn get_accounts(&self) -> DatabaseAdapterResult<Vec<AccountData>> {
        Ok(self.accounts.lock().await.iter().cloned().collect())
    }

    async fn get_account_by_name(&self, username: &str) -> DatabaseAdapterResult<AccountData> {
        self.accounts
            .lock().await
            .iter()
            .find(|data| data.username.as_str() == username)
            .map_or(Err(DatabaseAdapterError::UsernameNotFound), |data| {
                Ok(data.clone())
            })
    }

    async fn add_account(&self, new_account: AccountData) -> DatabaseAdapterResult<()> {
        let mut guard = self.accounts.lock().await;
        if guard.insert(new_account) {
            Ok(())
        } else {
            Err(DatabaseAdapterError::UsernameAlreadyExists)
        }
    }

    async fn remove_account_with_username(&self, username: &str) -> DatabaseAdapterResult<()> {
        let mut guard = self.accounts.lock().await;
        if guard.remove(username) {
            Ok(())
        } else {
            Err(DatabaseAdapterError::UsernameNotFound)
        }
    }

    async fn is_password_matching(
        &self,
        username: &str,
        password_plaintext: &str,
    ) -> DatabaseAdapterResult<bool> {
        let account = self.get_account_by_name(username).await?;
        account.verify(password_plaintext)
    }

    async fn change_password(
        &self,
        username: &str,
        old_password_plaintext: &str,
        new_password_plaintext: &str,
    ) -> DatabaseAdapterResult<()> {
        let old_password_is_matching = self
            .is_password_matching(username, old_password_plaintext)
            .await?;
        if old_password_is_matching {
            // Account already found, should not fail
            let new_account = {
                let mut tmp_account = self.get_account_by_name(username).await?;
                tmp_account.set_password(new_password_plaintext)?;
                tmp_account
            };

            //Update in HashSet: remove-insert again
            self.remove_account_with_username(username).await?;
            self.add_account(new_account).await
        } else {
            Err(DatabaseAdapterError::BadPassword)
        }
    }

    async fn get_accounts_count(&self) -> DatabaseAdapterResult<usize> {
        Ok(self.accounts.lock().await.len())
    }

    async fn get_characters(&self) -> DatabaseAdapterResult<Vec<CharacterData>> {
        Ok(
            self.characters_manager.lock().await
                .characters
                .iter()
                .cloned()
                .collect()
        )
    }

    async fn get_character_by_id(&self, character_id: CharacterId) -> DatabaseAdapterResult<CharacterData> {
        self.characters_manager
            .lock().await
            .characters
            .get(&character_id)
            .map_or(Err(DatabaseAdapterError::CharacterIdNotFound), |data| {
                Ok(data.clone())
            })
    }



    async fn add_character(&self, new_character: NewCharacterData) -> DatabaseAdapterResult<CharacterId> {
        let mut guard = self.characters_manager.lock().await;
        let assigned_character_id = guard.new_character_id;
        guard.new_character_id += 1;

        if guard.characters.insert(new_character.into_with_id(assigned_character_id)) {
            Ok(assigned_character_id)
        } else {
            Err(DatabaseAdapterError::CharacterAlreadyExists)
        }
    }

    async fn remove_character_with_id(&self, character_id: CharacterId) -> DatabaseAdapterResult<()> {
        let mut guard = self.characters_manager.lock().await;
        if guard.characters.remove(&character_id) {
            Ok(())
        } else {
            Err(DatabaseAdapterError::CharacterIdNotFound)
        }
    }

    async fn attach_character_to_account(&self, username: &str, character_id: CharacterId) -> DatabaseAdapterResult<()> {
        Ok(()) // TODO
    }

    async fn detach_character_from_account(&self, username: &str, character_id: CharacterId) -> DatabaseAdapterResult<()> {
        Ok(()) // TODO
    }

    async fn get_characters_of_account(&self, username: &str) -> DatabaseAdapterResult<()> {
        Ok(()) // TODO
    }
}

impl DatabaseTestAdapter {
    pub async fn new() -> Self {
        DatabaseTestAdapter {
            accounts: Mutex::new(HashSet::new()),
            characters_manager: Mutex::new(CharactersManager::new()),
        }
    }

    pub async fn with_test_data() -> Self {
        let db = Self::new().await;

        // Accounts
        db.add_account(AccountData::new("Account1".to_string(), "1234").unwrap()).await.unwrap();
        db.add_account(AccountData::new("Account2".to_string(), "1234").unwrap()).await.unwrap();

        // Characters
        let _ = db.add_character(NewCharacterData {
            name: "Janusz".to_string(),
            position_x: 0.0,
            position_y: 0.0,
            speed: 1.0
        }).await.unwrap();
        
        let _ = db.add_character(NewCharacterData {
            name: "Tuna".to_string(),
            position_x: 0.0,
            position_y: 1.0,
            speed: 2.0
        }).await.unwrap();

        let _ = db.add_character(NewCharacterData {
            name: "Raspberry".to_string(),
            position_x: -2.0,
            position_y: 0.0,
            speed: 1.2
        }).await.unwrap();
        
        db
    }
}

#[cfg(test)]
mod tests_accounts {
    use super::*;
    use crate::DatabaseAdapter;

    #[tokio::test]
    async fn test_appending_accounts_and_counting() {
        let db_adapter = DatabaseTestAdapter::new().await;
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 0);

        let account_1 = AccountData::new("User1".to_string(), "Password12345!@#").unwrap();
        let account_2 = AccountData::new("User2".to_string(), "Password12345!@#").unwrap();
        let account_3_username_already_used =
            AccountData::new(account_1.username.clone(), "Password12345!@#").unwrap();

        db_adapter.add_account(account_1).await.unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 1);

        db_adapter.add_account(account_2).await.unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 2);

        // AccountData with the same username
        let result = db_adapter
            .add_account(account_3_username_already_used)
            .await;
        assert!(
            matches!(result, Err(DatabaseAdapterError::UsernameAlreadyExists)),
            "result == {:?}",
            result
        );
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_removing_accounts() {
        let db_adapter = DatabaseTestAdapter::new().await;
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 0);

        let account_1 = AccountData::new("User1".to_string(), "Password12345!@#").unwrap();
        let account_2 = AccountData::new("User2".to_string(), "Password12345!@#").unwrap();

        db_adapter.add_account(account_1).await.unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 1);

        db_adapter.add_account(account_2).await.unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 2);

        assert!(matches!(
            db_adapter
                .remove_account_with_username("User")
                .await
                .unwrap_err(),
            DatabaseAdapterError::UsernameNotFound
        ));
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 2);

        db_adapter
            .remove_account_with_username("User1")
            .await
            .unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 1);

        db_adapter
            .remove_account_with_username("User2")
            .await
            .unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_changing_password() {
        let db_adapter = DatabaseTestAdapter::new().await;
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 0);

        let account_1 = AccountData::new("User1".to_string(), "Password12345!@#").unwrap();

        db_adapter.add_account(account_1).await.unwrap();
        assert_eq!(db_adapter.get_accounts_count().await.unwrap(), 1);

        db_adapter
            .change_password("User1", "Password12345!@#", "NewPassword12345!@#")
            .await
            .unwrap();

        db_adapter
            .change_password("User1", "NewPassword12345!@#", "NewerPassword12345!@#")
            .await
            .unwrap();

        let change_password_err_result = db_adapter
            .change_password("User1", "NewPassword12345!@#", "NewerPassword12345!@#")
            .await;

        assert!(matches!(
            change_password_err_result,
            Err(DatabaseAdapterError::BadPassword)
        ));
    }
}
