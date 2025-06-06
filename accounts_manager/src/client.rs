use std::time::Duration;
use reqwest::{Client as HttpClient, Response, StatusCode};
use crate::requests::AccountsServerStatus;

pub struct AccountsManagerClient {
    base_url: String,
    http_client: HttpClient,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountsManagerClientError {
    #[error("ReqwestError, reson = {0}")]
    ReqwestError(#[from] reqwest::Error),

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
}

#[cfg(test)]
mod tests {
    use crate::client::AccountsManagerClient;

    #[test]
    fn text_client_creation() {
        let _ = AccountsManagerClient::new("127.0.0.1:1234").unwrap();
    }
}