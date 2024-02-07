use tracing::{debug, instrument};

use crate::Result;
pub const BASE_URL: &str = "https://api.partner.market.yandex.ru/";

#[derive(Debug, serde::Deserialize)]
struct Config {
    access_token: String,
    check_token: String,
}
#[derive(Debug, serde::Deserialize)]
struct ConfigToml {
    config: Config,
}
/// Represents a client for interacting with the market.
///
/// # Example
///
/// ```rust
/// use rust_yandexmarket::{MarketClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = MarketClient::init().await?;
///     // Use the market_client instance...
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MarketClient {
    client: reqwest::Client,
    access_token: String,
    check_token: String,
    campaign_id: i64,
    business_id: i64,
}

impl MarketClient {
    /// Initializes a new `MarketClient` instance for first campaignId in a list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yandexmarket::{MarketClient, Result};
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let market_client = MarketClient::init().await?;
    ///     // Use the market_client instance...
    ///     Ok(())
    /// }
    /// ```
    /// require Config.toml file
    ///
    /// ```toml
    /// [config]
    /// access_token = 'someaccesstoken'
    /// check_token = 'somechecktoken'
    /// ```
    ///
    #[instrument]
    pub async fn init() -> Result<Self> {
        debug!("Initializing MarketClient");
        let file_path = "Config.toml";
        let mut file = std::fs::File::open(file_path)?;
        let mut str_val = String::new();
        std::io::Read::read_to_string(&mut file, &mut str_val)?;
        let config: ConfigToml = toml::from_str(&str_val)?;
        let client = reqwest::Client::builder().gzip(true).build()?;
        let mut result = MarketClient {
            client,
            access_token: config.config.access_token,
            check_token: config.config.check_token,
            campaign_id: 0,
            business_id: 0,
        };
        let campaigns = result.campaigns().get_all_campaigns().await?;
        let Some(campaign) = campaigns.first() else {
            return Err("No campaigns found".into());
        };
        result.campaign_id = campaign.id;
        result.business_id = campaign.business.id;
        Ok(result)
    }
    #[instrument]
    pub async fn from_env() -> Result<Self> {
        debug!("Initializing MarketClient");
        let access_token =
            std::env::var("MARKET_ACCESS_TOKEN").expect("MARKET_ACCESS_TOKEN env var not set!");
        let check_token =
            std::env::var("MARKET_CHECK_TOKEN").expect("MARKET_CHECK_TOKEN env var not set!");
        let client = reqwest::Client::builder().gzip(true).build()?;
        let mut result = MarketClient {
            client,
            access_token,
            check_token,
            campaign_id: 0,
            business_id: 0,
        };
        let campaigns = result.campaigns().get_all_campaigns().await?;
        let Some(campaign) = campaigns.first() else {
            return Err("No campaigns found".into());
        };
        result.campaign_id = campaign.id;
        result.business_id = campaign.business.id;
        Ok(result)
    }
    pub async fn with_tokens(
        access_token: impl Into<String>,
        check_token: impl Into<String>,
    ) -> Result<Self> {
        debug!("Initializing MarketClient");
        let client = reqwest::Client::builder().gzip(true).build()?;
        let mut result = MarketClient {
            client,
            access_token: access_token.into(),
            check_token: check_token.into(),
            campaign_id: 0,
            business_id: 0,
        };
        let campaigns = result.campaigns().get_all_campaigns().await?;
        let Some(campaign) = campaigns.first() else {
            return Err("No campaigns found".into());
        };
        result.campaign_id = campaign.id;
        result.business_id = campaign.business.id;
        Ok(result)
    }
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
    pub fn check_token(&self) -> &str {
        &self.check_token
    }
    pub fn campaign_id(&self) -> i64 {
        self.campaign_id
    }
    pub fn business_id(&self) -> i64 {
        self.business_id
    }
}
