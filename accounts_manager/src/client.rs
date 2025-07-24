use crate::requests::{CreateAccountRequest, LoginAccountRequest, NewCharacterRequest, UpdatePasswordRequest};
use crate::responses::{AccountDetails, AccountsServerStatus, ApiError};
use reqwest::{Client as HttpClient, Response, StatusCode};
use std::time::Duration;
use database_adapter::character::{CharacterData, CharacterId};
use crate::{JwtToken};

pub struct AccountsManagerClient {
    base_url: String,
    http_client: HttpClient,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountsManagerClientError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ApiError(#[from] ApiError),

    #[error("OtherError, status_code={status}, reason = {reason}")]
    OtherError { status: StatusCode, reason: String },

    #[error("Timeout")]
    Timeout,

    #[error("Unauthorized")]
    Unauthorized,
}

pub type AccountsManagerClientResult<T> = Result<T, AccountsManagerClientError>;

impl AccountsManagerClient {
    pub fn new(address: &str) -> AccountsManagerClientResult<Self> {
        let base_url = format!("http://{}", address.trim_end_matches('/'));
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        tracing::info!("Created client with base_url='{base_url}'.");

        Ok(Self {
            base_url,
            http_client,
        })
    }

    pub async fn get_server_status(&self) -> AccountsManagerClientResult<AccountsServerStatus> {
        let resp = self.http_client
            .get(&format!("{}/", self.base_url))
            .send().await?
            .error_for_status()?;

        resp.json().await.map_err(Into::into)
    }

    pub async fn request_create_account(
        &self,
        username: String,
        password: String,
    ) -> AccountsManagerClientResult<()> {
        let request_payload = CreateAccountRequest { username, password };
        let resp = self.http_client
            .post(&format!("{}/api/account/create", self.base_url))
            .json(&request_payload)
            .send().await?;

        match resp.status() {
            StatusCode::CREATED => Ok(()),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }

    pub async fn request_login_to_account(
        &self,
        username: String,
        password: String,
    ) -> AccountsManagerClientResult<JwtToken> {
        let request_payload = LoginAccountRequest { password };
        let resp = self.http_client
            .post(&format!("{}/api/accounts/{}/login", self.base_url, username))
            .json(&request_payload)
            .send().await?;
        
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }

    pub async fn request_logout_account(
        &self,
        username: String,
        token: JwtToken,
    ) -> AccountsManagerClientResult<()> {
        let url = format!("{}/api/accounts/{}/logout", self.base_url, username);
        let resp = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send().await?;

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AccountsManagerClientError::Unauthorized),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }

    pub async fn request_account_details(
        &self,
        username: String,
        token: &JwtToken,
    ) -> AccountsManagerClientResult<AccountDetails> {
        let resp = self.http_client
            .get(&format!("{}/api/accounts/{}", self.base_url, username))
            .header("Authorization", format!("Bearer {}", token))
            .send().await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            StatusCode::UNAUTHORIZED => Err(AccountsManagerClientError::Unauthorized),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }

    pub async fn request_delete_account(
        &self,
        username: String,
        token: JwtToken,
    ) -> AccountsManagerClientResult<()> {
        let resp = self.http_client
            .delete(&format!("{}/api/accounts/{}", self.base_url, username))
            .header("Authorization", format!("Bearer {}", token))
            .send().await?;

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AccountsManagerClientError::Unauthorized),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }


    pub async fn request_update_account_password(
        &self,
        username: String,
        password_old: String,
        password_new: String,
        token: &JwtToken,
    ) -> AccountsManagerClientResult<()> {
        let request_payload = UpdatePasswordRequest { password_old, password_new };
        let resp = self.http_client
            .patch(&format!("{}/api/accounts/{}/password", self.base_url, username))
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_payload)
            .send().await?;

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AccountsManagerClientError::Unauthorized),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }


    pub async fn request_create_character(
        &self,
        username: String,
        character_name: String,
        token: &JwtToken,
    ) -> AccountsManagerClientResult<CharacterId> {
        let request_payload = NewCharacterRequest { character_name };
        let resp = self.http_client
            .post(&format!("{}/api/accounts/{}/character/new", self.base_url, username))
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_payload)
            .send().await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            StatusCode::UNAUTHORIZED => Err(AccountsManagerClientError::Unauthorized),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }

    pub async fn request_account_characters(
        &self,
        username: String,
        token: &JwtToken,
    ) -> AccountsManagerClientResult<Vec<CharacterData>> {
        let resp = self.http_client
            .get(&format!("{}/api/accounts/{}/characters", self.base_url, username))
            .header("Authorization", format!("Bearer {}", token))
            .send().await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            StatusCode::UNAUTHORIZED => Err(AccountsManagerClientError::Unauthorized),
            status => {
                let reason = resp.text().await?;
                Err(
                    match serde_json::from_str::<ApiError>(&reason) {
                        Ok(err) => err.into(),
                        Err(_) => AccountsManagerClientError::OtherError { status, reason },
                    }
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{AccountsManagerClient, AccountsManagerClientError};

    #[test]
    fn text_client_creation_not_connected_yet() {
        let _ = AccountsManagerClient::new("127.0.0.1:1234").unwrap();
    }

    #[tokio::test]
    async fn text_client_requesting_not_existing_server() {
        let dummy_client = AccountsManagerClient::new("127.255.255.255:1234").unwrap();
        let request_err = dummy_client.get_server_status().await.unwrap_err();

        match request_err {
            AccountsManagerClientError::ReqwestError(reqwest_err) => {
                assert!(reqwest_err.is_connect()); // Watch out, this dude returns true if error is connection related lol
                assert!(format!("{reqwest_err:?}").contains("ConnectError"));
            }
            _ => panic!("Should not get ReqwestError!"),
        }
    }
}
