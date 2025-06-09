use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("PasswordHashError reason= {0}")]
    PasswordHashError(String), // argon2::password_hash::Error is not std::error::Error based

    #[error("UsernameAlreadyExists")]
    UsernameAlreadyExists,

    #[error("UsernameNotFound")]
    UsernameNotFound,
}

#[derive(Debug)]
pub struct Account {
    pub username: String,
    hashed_password: String,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.username.eq(&other.username)
    }
}

impl Eq for Account {}

impl Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state)
    }
}

impl Account {
    pub fn new(username: String, plaintext_password: &str) -> Result<Account, AccountError> {
        let hashed_password = hash_password(plaintext_password)
            .map_err(|e| AccountError::PasswordHashError(e.to_string()))?;

        Ok(Self {
            username,
            hashed_password,
        })
    }

    pub fn verify(&self, password_plaintext: &str) -> Result<bool, AccountError> {
        verify_password(&self.hashed_password, password_plaintext)
            .map_err(|e| AccountError::PasswordHashError(e.to_string()))
    }
}

#[derive(Debug)]
pub struct AccountsManager {
    accounts: HashSet<Account>,
}

impl AccountsManager {
    pub fn new() -> Self {
        Self {
            accounts: HashSet::new(),
        }
    }

    fn find_account_by_username(&self, username: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|account| account.username == username)
    }

    pub fn add_account(&mut self, new_account: Account) -> Result<(), AccountError> {
        if self.accounts.insert(new_account) {
            Ok(())
        } else {
            Err(AccountError::UsernameAlreadyExists)
        }
    }
}

fn hash_password(password_plaintext: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hashed = Argon2::default()
        .hash_password(password_plaintext.as_bytes(), &salt)?
        .to_string();

    Ok(password_hashed)
}

fn verify_password(
    hashed_stored_password: &str,
    candidate_plain_password: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hashed_stored_password)?;

    match Argon2::default().verify_password(candidate_plain_password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::account::{hash_password, verify_password, Account, AccountError, AccountsManager};

    #[test]
    fn test_hashing_empty_password() {
        let password_empty_hased_stored = hash_password("").unwrap();
        assert!(!password_empty_hased_stored.is_empty());
    }
    #[test]
    fn test_hashing_the_same_password_should_give_different_results() {
        let password_1 = "Hello1234%^&";
        let password_1_hased_stored = hash_password(password_1).unwrap();
        let password_2_hased_stored = hash_password(password_1).unwrap();
        assert_ne!(password_1, password_1_hased_stored);
        assert_ne!(password_1, password_2_hased_stored);
        assert_ne!(password_1_hased_stored, password_2_hased_stored);
    }

    #[test]
    fn test_verify_password_should_match() {
        let password_1 = "Hello1234%^&";
        let password_2 = password_1;
        let password_1_hased_stored = hash_password(password_1).unwrap();
        let passwords_matches = verify_password(&password_1_hased_stored, password_2).unwrap();
        assert!(passwords_matches);
    }

    #[test]
    fn test_verify_password_should_not_match() {
        let password_1 = "Hello1234%^&";
        let password_2 = "Hello1234%^";
        let password_1_hased_stored = hash_password(password_1).unwrap();
        let passwords_matches = verify_password(&password_1_hased_stored, password_2).unwrap();
        assert!(!passwords_matches);
    }

    #[test]
    fn test_creating_account() {
        let _account = Account::new("User".to_string(), "Password").unwrap();
    }

    #[test]
    fn test_verifying_account_password_should_match() {
        let password = "Password";
        let account = Account::new("User".to_string(), password).unwrap();
        let password_matches = account.verify(password).unwrap();
        assert!(password_matches);
    }

    #[test]
    fn test_verifying_account_password_should_not_match() {
        let password_1 = "Password";
        let password_2 = "password";
        let account = Account::new("User".to_string(), password_1).unwrap();
        let password_matches = account.verify(password_2).unwrap();
        assert!(!password_matches);
    }

    #[test]
    fn test_accounts_manager_appending_accounts() {
        let mut accounts_manager = AccountsManager::new();

        let account_1 = Account::new("User1".to_string(), "Password12345!@#").unwrap();
        let account_2 = Account::new("User2".to_string(), "Password12345!@#").unwrap();
        let account_3_username_already_used =
            Account::new(account_1.username.clone(), "Password12345!@#").unwrap();

        accounts_manager.add_account(account_1).unwrap();
        accounts_manager.add_account(account_2).unwrap();

        // Account with the same username
        let result = accounts_manager.add_account(account_3_username_already_used);
        assert!(matches!(result, Err(AccountError::UsernameAlreadyExists)), "result == {:?}", result);
    }
}
