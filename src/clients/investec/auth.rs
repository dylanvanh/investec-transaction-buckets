use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::settings::Config;
use anyhow::Result;
use reqwest::Client;

use super::models::TokenResponse;

#[derive(Debug)]
pub struct TokenState {
    pub access_token: String,
    pub expires_at: u64,
}

pub struct Authenticator {
    http: Client,
    pub api_key: String,
    client_id: String,
    client_secret: String,
    token: Arc<Mutex<TokenState>>,
}

impl Authenticator {
    pub fn new(config: &Config) -> Self {
        Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
            api_key: config.x_api_key.clone(),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            token: Arc::new(Mutex::new(TokenState {
                access_token: String::new(),
                expires_at: 0,
            })),
        }
    }

    pub fn is_token_expired(&self) -> bool {
        let token_state = self.token.lock().unwrap();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let five_minutes_buffer = 5 * 60;
        token_state.expires_at.saturating_sub(current_time) < five_minutes_buffer
    }

    pub async fn authenticate(&self) -> Result<()> {
        let response = self
            .http
            .post("https://openapi.investec.com/identity/v2/oauth2/token")
            .header("x-api-key", &self.api_key)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("scope", "accounts"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let body = response.text().await?;
            return Err(anyhow::anyhow!("Authentication failed: {}", body));
        }

        let token_response: TokenResponse = response.json().await?;

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + token_response.expires_in;

        let mut token_state = self.token.lock().unwrap();
        token_state.access_token = token_response.access_token;
        token_state.expires_at = expires_at;

        Ok(())
    }

    pub async fn get_valid_token(&self) -> Result<String> {
        if self.is_token_expired() {
            self.authenticate().await?;
        }

        let token_state = self.token.lock().unwrap();
        Ok(token_state.access_token.clone())
    }
}
