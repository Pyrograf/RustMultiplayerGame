use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use crate::{DatabaseAdapterError, DatabaseAdapterResult};

#[derive(Debug, Clone)]
pub struct AccountData {
    pub username: String,
    pub hashed_password: String,
}

impl PartialEq for AccountData {
    fn eq(&self, other: &Self) -> bool {
        self.username.eq(&other.username)
    }
}

impl Eq for AccountData {}

impl Hash for AccountData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state)
    }
}

impl Borrow<str> for AccountData {
    fn borrow(&self) -> &str {
        self.username.as_str()
    }
}

impl AccountData {
    pub fn new(username: String, password_plaintext: &str) -> DatabaseAdapterResult<AccountData> {
        let hashed_password = hash_password(password_plaintext)
            .map_err(|e| DatabaseAdapterError::PasswordHashError(e.to_string()))?;

        Ok(Self {
            username,
            hashed_password,
        })
    }

    pub fn verify(&self, password_plaintext: &str) -> DatabaseAdapterResult<bool> {
        verify_password(&self.hashed_password, password_plaintext)
            .map_err(|e| DatabaseAdapterError::PasswordHashError(e.to_string()))
    }

    pub fn set_password(&mut self, password_plaintext: &str) -> DatabaseAdapterResult<()> {
        self.hashed_password = hash_password(password_plaintext)
            .map_err(|e| DatabaseAdapterError::PasswordHashError(e.to_string()))?;
        Ok(())
    }
}

pub fn hash_password(password_plaintext: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hashed = Argon2::default()
        .hash_password(password_plaintext.as_bytes(), &salt)?
        .to_string();

    Ok(password_hashed)
}

pub fn verify_password(
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
    use crate::account::{hash_password, verify_password};
    use crate::AccountData;

    #[test]
    fn test_hashing_empty_password() {
        let hashed_password_empty = hash_password("").unwrap();
        assert!(!hashed_password_empty.is_empty());
    }

    #[test]
    fn test_hashing_the_same_password_should_give_different_results() {
        let password_1 = "Hello1234%^&";
        let password_1_hashed = hash_password(password_1).unwrap();
        let password_2_hashed = hash_password(password_1).unwrap();
        assert_ne!(password_1, password_1_hashed);
        assert_ne!(password_1, password_2_hashed);
        assert_ne!(password_1_hashed, password_2_hashed);
    }

    #[test]
    fn test_verify_password_should_match() {
        let password_1 = "Hello1234%^&";
        let password_2 = password_1;
        let password_1_hashed = hash_password(password_1).unwrap();
        let passwords_matches = verify_password(&password_1_hashed, password_2).unwrap();
        assert!(passwords_matches);
    }

    #[test]
    fn test_verify_password_should_not_match() {
        let password_1 = "Hello1234%^&";
        let password_2 = "Hello1234%^";
        let password_1_hashed = hash_password(password_1).unwrap();
        let passwords_matches = verify_password(&password_1_hashed, password_2).unwrap();
        assert!(!passwords_matches);
    }

    #[test]
    fn test_creating_account() {
        let _account = AccountData::new("User".to_string(), "Password").unwrap();
    }

    #[test]
    fn test_verifying_account_password_should_match() {
        let password = "Password";
        let account = AccountData::new("User".to_string(), password).unwrap();
        let password_matches = account.verify(password).unwrap();
        assert!(password_matches);
    }

    #[test]
    fn test_verifying_account_password_should_not_match() {
        let password_1 = "Password";
        let password_2 = "password";
        let account = AccountData::new("User".to_string(), password_1).unwrap();
        let password_matches = account.verify(password_2).unwrap();
        assert!(!password_matches);
    }

    #[test]
    fn test_set_account_password() {
        let password_1 = "Password1";
        let password_2 = "Password2";
        let mut account = AccountData::new("User".to_string(), password_1).unwrap();

        let password_matches = account.verify(password_1).unwrap();
        assert!(password_matches);

        let password_matches = account.verify(password_2).unwrap();
        assert!(!password_matches);

        // Set Password check again
        account.set_password(password_2).unwrap();

        let password_matches = account.verify(password_1).unwrap();
        assert!(!password_matches);

        let password_matches = account.verify(password_2).unwrap();
        assert!(password_matches);
    }

}