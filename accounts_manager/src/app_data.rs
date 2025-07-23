use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use axum_jwt_auth::{JwtDecoderState, LocalDecoder};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use database_adapter::DatabaseAdapter;
use crate::SERVICE_AUDIENCE;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountManagerClaims {
    pub iss: String,        // Optional. Issuer
    pub iat: u64,           // Optional. Issued at (as UTC timestamp)
    pub aud: String,        // Optional. Audience
    pub exp: u64,           // Required. Expiration time (as UTC timestamp)
}

pub struct AppData {
    pub database_adapter: Arc<dyn DatabaseAdapter>,
    pub jwt_decoder: JwtDecoderState<AccountManagerClaims>,
}

impl AppData {
    pub async fn new(database_adapter: Arc<dyn DatabaseAdapter>) -> Self {
        let public_key = database_adapter.get_jwt_public_key().await.unwrap();
        let keys = vec![DecodingKey::from_rsa_pem(&public_key).unwrap()];

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[SERVICE_AUDIENCE]);

        let local_jwt_decoder = LocalDecoder::builder()
            .keys(keys)
            .validation(validation)
            .build()
            .unwrap();

        let jwt_decoder = JwtDecoderState {
            decoder: Arc::new(local_jwt_decoder),
        };

        Self {
            database_adapter,
            jwt_decoder,
        }
    }
}

impl Debug for AppData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppData")
    }
}