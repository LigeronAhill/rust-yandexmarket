use crate::{models::campaigns::CampaignDTO, Result};
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct Config {
    access_token: String,
    check_token: String,
    base_url: String,
}
#[allow(dead_code)]
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
    base_url: String,
    campaigns: Vec<CampaignDTO>,
}

impl MarketClient {
    /// Initializes a new `MarketClient` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    /// base_url = "https://api.partner.market.yandex.ru/"
    /// ```
    ///
    pub async fn init() -> Result<Self> {
        let file_path = "Config.toml";
        let mut file = match std::fs::File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };
        let mut str_val = String::new();
        match std::io::Read::read_to_string(&mut file, &mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        let config: ConfigToml = toml::from_str(&str_val).unwrap();
        let client = reqwest::Client::builder().gzip(true).build()?;
        let mut result = MarketClient {
            client,
            access_token: config.config.access_token,
            check_token: config.config.check_token,
            base_url: config.config.base_url,
            campaigns: vec![],
        };
        let campaigns = result.get_all_campaigns().await?;
        result.campaigns = campaigns;
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
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn campaigns(&self) -> &Vec<CampaignDTO> {
        &self.campaigns
    }
}
