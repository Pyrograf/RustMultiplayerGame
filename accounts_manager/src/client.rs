use std::time::Duration;
use reqwest::{Client as HttpClient, Response, StatusCode};
use crate::account::AccountError;
use crate::requests::CreateAccountRequest;
use crate::responses::AccountsServerStatus;

pub struct AccountsManagerClient {
    base_url: String,
    http_client: HttpClient,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountsManagerClientError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    AccountError(#[from] AccountError),

    #[error("OtherError, reason = {0}")]
    OtherError(String),

}

impl AccountsManagerClient {
    pub fn new(address: &str) -> Result<Self, AccountsManagerClientError> {
        let base_url = format!("http://{}", address.trim_end_matches('/'));
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        tracing::info!("Creatied client with base_url='{base_url}'.");

        Ok(
            Self {
                base_url,
                http_client
            }
        )
    }

    pub async fn get_server_status(&self) -> Result<AccountsServerStatus, AccountsManagerClientError> {
        let url = format!("{}/", self.base_url);
        let resp = self.http_client.get(&url).send().await?.error_for_status()?;
        let status = resp.json::<AccountsServerStatus>().await?;
        Ok(status)
    }

    pub async fn request_create_account(
        &self,
        username: String,
        password: String
    ) -> Result<(), AccountsManagerClientError> {
        let url = format!("{}/api/account/create", self.base_url);
        let request_payload = CreateAccountRequest {
            username,
            password
        };
        let resp= self.http_client
            .post(&url)
            .json(&request_payload)
            .send().await?;

        let status =  resp.status();

        match resp.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT | StatusCode::BAD_REQUEST | StatusCode::INTERNAL_SERVER_ERROR => {
                // Try to parse JSON error if possible
                let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());

                // Optional: deserialize to AccountError if your server returns structured errors
                // let account_error: Result<AccountError, _> = serde_json::from_str(&error_text);
                // match account_error {
                //     Ok(err) => Err(AccountsManagerClientError::AccountError(err)),
                //     Err(_) => Err(AccountsManagerClientError::OtherError(error_text)),
                // }
                // TODO
                Err(AccountsManagerClientError::OtherError(error_text))
            }
            _ => {
                let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(AccountsManagerClientError::OtherError(format!(
                    "Unexpected status code {}: {}",
                    status,
                    error_text
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::AccountsManagerClient;

    #[test]
    fn text_client_creation() {
        let _ = AccountsManagerClient::new("127.0.0.1:1234").unwrap();
    }
}