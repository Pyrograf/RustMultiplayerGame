use std::collections::HashSet;
use crate::account::{Account, AccountsManager};

#[derive(Debug)]
pub struct AppData {
    accounts_manager: AccountsManager,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            accounts_manager:  AccountsManager::new(),
        }
    }
}