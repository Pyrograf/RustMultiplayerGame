pub mod account;
pub mod test;

use std::future::Future;
use serde::{Deserialize, Serialize};
pub use account::AccountData;

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
}

pub type  DatabaseAdapterResult<T> = Result<T, DatabaseAdapterError>;

pub trait DatabaseAdapter {
    fn get_accounts(&self) -> impl Future<Output = DatabaseAdapterResult<Vec<AccountData>>>;

    fn get_account_by_name(&self, username: &str) -> impl Future<Output = DatabaseAdapterResult<AccountData>>;

    fn add_account(&self, new_account: AccountData) -> impl Future<Output = DatabaseAdapterResult<()>>;

    fn remove_account_with_username(&self, username: &str) -> impl Future<Output = DatabaseAdapterResult<()>>;

    fn is_password_matching(&self, username: &str, password_plaintext: &str) -> impl Future<Output = DatabaseAdapterResult<bool>>;

    fn change_password(&self, username: &str, old_password_plaintext: &str, new_password_plaintext: &str) -> impl Future<Output = DatabaseAdapterResult<()>>;

    fn get_accounts_count(&self) -> impl Future<Output = DatabaseAdapterResult<usize>>;
}
