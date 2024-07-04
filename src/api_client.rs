use std::fmt::Debug;

use anyhow::Result;
use reqwest::header::{ACCEPT, HeaderMap};
use tracing::{debug, instrument};
use url::Url;

use crate::models::CampaignDto;

pub const BASE_URL: &str = "https://api.partner.market.yandex.ru/";

/// Represents a client for interacting with the market.
///
/// # Example
///
/// ```rust
/// use rust_yandexmarket::MarketClient;
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // get token from env or config file...
///     let access_token = "somE-toKen";
///     let client = MarketClient::init(access_token).await?;
///     // Use the market_client instance...
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MarketClient {
    base_path: Url,
    client: reqwest::Client,
    access_token: String,
    campaign_ids: Vec<CampaignDto>,
}

impl MarketClient {
    /// Initializes a new `MarketClient` instance for first campaignId in a list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yandexmarket::MarketClient;
    /// use anyhow::Result;
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     // get token from env or config file...
    ///     let access_token = "somE-toKen";
    ///     let market_client = MarketClient::init(access_token).await?;
    ///     // Use the market_client instance...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn init<T: AsRef<str> + Debug>(access_token: T) -> Result<Self> {
        debug!("Initializing MarketClient");
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let mut header = HeaderMap::new();
        header.insert(ACCEPT, "application/json".parse()?);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(header)
            .gzip(true)
            .build()?;
        let base_path = BASE_URL.parse()?;
        let mut result = MarketClient {
            base_path,
            client,
            access_token: access_token.as_ref().into(),
            campaign_ids: Vec::new(),
        };
        let campaigns = result.get_campaigns().await?;
        result.campaign_ids = campaigns;
        Ok(result)
    }
    #[instrument]
    pub async fn from_env() -> Result<Self> {
        debug!("Initializing MarketClient");
        let access_token =
            std::env::var("MARKET_ACCESS_TOKEN").expect("MARKET_ACCESS_TOKEN env var not set!");
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let mut header = HeaderMap::new();
        header.insert(ACCEPT, "application/json".parse()?);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(header)
            .gzip(true)
            .build()?;
        let base_path = BASE_URL.parse()?;
        let access_token: &str = access_token.as_ref();
        let mut result = MarketClient {
            base_path,
            client,
            access_token: access_token.to_string(),
            campaign_ids: Vec::new(),
        };
        let campaigns = result.get_campaigns().await?;
        result.campaign_ids = campaigns;
        Ok(result)
    }
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
    pub fn campaigns(&self) -> Vec<CampaignDto> {
        self.campaign_ids.clone()
    }
    pub fn base_path(&self) -> &Url {
        &self.base_path
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let client = MarketClient::from_env().await?;
        assert!(client.campaigns().len() > 0);
        Ok(())
    }
}