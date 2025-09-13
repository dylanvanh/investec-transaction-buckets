use crate::config::settings::Config;
use anyhow::Result;
use reqwest::Client;
use url::Url;

use super::auth::Authenticator;
use super::models::{Account, AccountsResponse, ApiResponse, Balance, TransactionsResponse};

const API_KEY_HEADER: &str = "x-api-key";

pub struct InvestecClient {
    http: Client,
    base: Url,
    authenticator: Authenticator,
}

impl InvestecClient {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()?,
            base: Url::parse("https://openapi.investec.com")?,
            authenticator: Authenticator::new(&config),
        })
    }

    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        let token = self.authenticator.get_valid_token().await?;
        let url = self.base.join("za/pb/v1/accounts")?;

        let response = self
            .http
            .get(url)
            .header(API_KEY_HEADER, &self.authenticator.api_key)
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow::anyhow!(
                "API request failed with status {}: {}",
                status,
                body
            ));
        }

        let body = response.text().await?;

        let api_response: ApiResponse<AccountsResponse> = serde_json::from_str(&body)?;
        Ok(api_response.data.accounts)
    }

    pub async fn get_balance(&self, account_id: &str) -> Result<Balance> {
        let token = self.authenticator.get_valid_token().await?;
        let url = self
            .base
            .join(&format!("za/pb/v1/accounts/{}/balance", account_id))?;

        let response = self
            .http
            .get(url)
            .header(API_KEY_HEADER, &self.authenticator.api_key)
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow::anyhow!(
                "Balance API request failed with status {}: {}",
                status,
                body
            ));
        }

        let body = response.text().await?;

        let api_response: ApiResponse<Balance> = serde_json::from_str(&body)?;
        Ok(api_response.data)
    }

    pub async fn get_transactions(
        &self,
        account_id: &str,
        from_date: &str,
        to_date: &str,
    ) -> Result<TransactionsResponse> {
        let token = self.authenticator.get_valid_token().await?;
        let url = self
            .base
            .join(&format!("za/pb/v1/accounts/{}/transactions", account_id))?;

        let response = self
            .http
            .get(url)
            .header(API_KEY_HEADER, &self.authenticator.api_key)
            .bearer_auth(token)
            .query(&[("fromDate", from_date), ("toDate", to_date)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow::anyhow!(
                "Transactions API request failed with status {}: {}",
                status,
                body
            ));
        }

        let body = response.text().await?;

        let api_response: ApiResponse<TransactionsResponse> = serde_json::from_str(&body)?;
        Ok(api_response.data)
    }
}
