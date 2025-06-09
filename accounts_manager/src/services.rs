use crate::account::{Account, AccountError, AccountsManager};

pub fn create_account(
    username: String, 
    password: String, 
    accounts_manager: &mut AccountsManager
) -> Result<(), AccountError> {
    let new_account = Account::new(username, &password)?;
    accounts_manager.add_account(new_account)?;
    Ok(())
}

pub fn delete_account(
    username: String,
    accounts_manager: &mut AccountsManager
) -> Result<(), AccountError> {
    accounts_manager.remove_account(&username)?;
    Ok(())
}