use crate::{AccountData, DatabaseAdapter, DatabaseAdapterError, DatabaseAdapterResult};
use std::collections::HashSet;
use async_trait::async_trait;
use tokio::sync::Mutex;

pub struct DatabaseTestAdapter {
    accounts: Mutex<HashSet<AccountData>>,
}

#[async_trait]
impl DatabaseAdapter for DatabaseTestAdapter {
    async fn get_accounts(&self) -> DatabaseAdapterResult<Vec<AccountData>> {
        Ok(self.accounts.lock().await.iter().cloned().collect())
    }

    async fn get_account_by_name(&self, username: &str) -> DatabaseAdapterResult<AccountData> {
        self.accounts
            .lock()
            .await
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
}

impl DatabaseTestAdapter {
    pub async fn new() -> Self {
        DatabaseTestAdapter {
            accounts: Mutex::new(HashSet::new()),
        }
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
