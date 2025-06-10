use crate::account::{Account, AccountError, AccountResult, AccountsManager};

fn verify_password(
    username: &str,
    password: &str,
    accounts_manager: &AccountsManager
) -> AccountResult<()> {
    let account = accounts_manager
        .find_account_by_username(&username)
        .ok_or(AccountError::UsernameNotFound)?;
    let password_matches = account.verify(&password)?;
    if !password_matches {
        return Err(AccountError::BadPassword);
    } else {
        Ok(())
    }
}

pub fn create_account(
    username: String,
    password: String,
    accounts_manager: &mut AccountsManager,
) -> AccountResult<()> {
    let new_account = Account::new(username, &password)?;
    accounts_manager.add_account(new_account)?;
    Ok(())
}

pub fn delete_account(
    username: String,
    password: String,
    accounts_manager: &mut AccountsManager,
) -> AccountResult<()> {
    verify_password(&username, &password, &accounts_manager)?;
    accounts_manager.remove_account(&username)?;
    Ok(())
}

pub fn update_account_password(
    username: String,
    password_old: String,
    password_new: String,
    accounts_manager: &mut AccountsManager,
) -> AccountResult<()> {
    verify_password(&username, &password_old, &accounts_manager)?;

    // Take account
    let mut account = accounts_manager.take_account(&username)
        .expect("Account should be found");

    // Replace password
    account.set_password(&password_new)?;

    //Place account back
    accounts_manager.add_account(account)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creating_account() {
        let mut accounts_manager = AccountsManager::new();

        create_account(
            "User1".to_string(),
            "psswdrad123456%^&*".to_string(),
            &mut accounts_manager,
        )
        .unwrap();

        create_account(
            "User2".to_string(),
            "psswdrad123456%^&*".to_string(),
            &mut accounts_manager,
        )
        .unwrap();

        create_account(
            "User".to_string(),
            "psswdrad123456%^&*".to_string(),
            &mut accounts_manager,
        )
        .unwrap();

        let err_user_alredy_exists = create_account(
            "User1".to_string(),
            "psswdrad123456%^&*".to_string(),
            &mut accounts_manager,
        )
        .unwrap_err();

        assert!(
            matches!(err_user_alredy_exists, AccountError::UsernameAlreadyExists),
            "Err {:?}",
            err_user_alredy_exists
        );
        assert_eq!(accounts_manager.count(), 3);
    }

    #[test]
    fn test_delete_account() {
        let mut accounts_manager = AccountsManager::new();

        let account_1 = Account::new("User11".to_string(), "qwertyuio12345$%^&").unwrap();
        accounts_manager.add_account(account_1).unwrap();
        assert_eq!(accounts_manager.count(), 1);

        let account_2_user = "User12";
        let account_2_psswrd = "mehmeh12345$%^&";
        let account_2 = Account::new(account_2_user.to_string(), account_2_psswrd).unwrap();
        accounts_manager.add_account(account_2).unwrap();
        assert_eq!(accounts_manager.count(), 2);

        delete_account(
            account_2_user.to_string(),
            account_2_psswrd.to_string(),
            &mut accounts_manager,
        )
        .unwrap();
        assert_eq!(accounts_manager.count(), 1);

        let err_user_not_found = delete_account(
            account_2_user.to_string(),
            account_2_psswrd.to_string(),
            &mut accounts_manager,
        )
        .unwrap_err();

        assert!(
            matches!(err_user_not_found, AccountError::UsernameNotFound),
            "Err {:?}",
            err_user_not_found
        );
        assert_eq!(accounts_manager.count(), 1);
    }
}
