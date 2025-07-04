pub mod account;
pub mod test;
mod character;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
pub use account::AccountData;
use crate::character::{CharacterData, CharacterId};

#[derive(Debug, thiserror::Error, Serialize, Deserialize, PartialEq, Clone)]
pub enum DatabaseAdapterError {
    #[error("Password hash error, reason = '{0}'")]
    PasswordHashError(String), // argon2::password_hash::Error is not std::error::Error based

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Username not found")]
    UsernameNotFound,

    #[error("Bad password")]
    BadPassword,

    #[error("Character ID not found")]
    CharacterIdNotFound,

    #[error("Character already exists")]
    CharacterAlreadyExists,
}

pub type  DatabaseAdapterResult<T> = Result<T, DatabaseAdapterError>;

#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn get_accounts(&self) -> DatabaseAdapterResult<Vec<AccountData>>;

    async fn get_account_by_name(&self, username: &str) -> DatabaseAdapterResult<AccountData>;

    async fn add_account(&self, new_account: AccountData) -> DatabaseAdapterResult<()>;

    async fn remove_account_with_username(&self, username: &str) -> DatabaseAdapterResult<()>;

    async fn is_password_matching(&self, username: &str, password_plaintext: &str) -> DatabaseAdapterResult<bool>;

    async fn change_password(&self, username: &str, old_password_plaintext: &str, new_password_plaintext: &str) -> DatabaseAdapterResult<()>;

    async fn get_accounts_count(&self) -> DatabaseAdapterResult<usize>;

    async fn get_characters(&self) -> DatabaseAdapterResult<Vec<CharacterData>>;

    async fn get_character_by_id(&self, character_id: CharacterId) -> DatabaseAdapterResult<CharacterData>;

    async fn add_character(&self, new_character: CharacterData) -> DatabaseAdapterResult<()>;

    async fn remove_character_with_id(&self, character_id: CharacterId) -> DatabaseAdapterResult<()>;

    async fn attach_character_to_account(&self, username: &str, character_id: CharacterId) -> DatabaseAdapterResult<()>;

    async fn detach_character_from_account(&self, username: &str, character_id: CharacterId) -> DatabaseAdapterResult<()>;

    async fn get_characters_of_account(&self, username: &str) -> DatabaseAdapterResult<()>;
}
