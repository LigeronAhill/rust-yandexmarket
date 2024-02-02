use tracing::{debug, instrument};

use crate::{
    models::{
        campaigns::{
            CampaignDTO, CampaignResponse, CampaignSettingsDTO, CampaignsResponse, LoginsResponse,
            SettingsResponse,
        },
        ErrorResponse,
    },
    MarketClient, Result,
};
#[derive(Debug, Clone)]
pub struct Campaigns<'a> {
    api_client: &'a MarketClient,
}
impl<'a> Campaigns<'a> {
    /// Возвращает список магазинов, к которым имеет доступ пользователь — владелец
    /// авторизационного токена, использованного в запросе.
    /// Для агентских пользователей
    /// список состоит из подагентских магазинов.
    #[instrument]
    pub async fn get_all_campaigns(&self) -> Result<Vec<CampaignDTO>> {
        let uri = format!("{}campaigns", crate::BASE_URL);
        debug!("Getting campaigns");
        let result = self.get_campaigns_with_uri(uri).await?;
        Ok(result)
    }
    /// Информация о магазине с заданным id
    #[instrument]
    pub async fn get_campaign(&self, campaign_id: i64) -> Result<CampaignDTO> {
        let uri = format!("{}campaigns/{campaign_id}", crate::BASE_URL);
        debug!("Getting campaign with id: {campaign_id}");
        let response = self
            .api_client
            .client()
            .get(&uri)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let campaign: CampaignResponse = response.json().await?;
                Ok(campaign.campaign)
            }
            _ => {
                let error: ErrorResponse = response.json().await?;
                let msg =
                    format!("Error while getting campaign with id {campaign_id} --> \n{error:#?}");
                return Err(msg.into());
            }
        }
    }
    /// Возвращает список логинов, у которых есть доступ к магазину.
    #[instrument]
    pub async fn get_logins(&self, campaign_id: i64) -> Result<Vec<String>> {
        let uri = format!("{}campaigns/{campaign_id}/logins", crate::BASE_URL);
        debug!("Getting campaign with id: {campaign_id} logins");
        let response = self
            .api_client
            .client()
            .get(&uri)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let logins: LoginsResponse = response.json().await?;
                Ok(logins.logins)
            }
            _ => {
                let error: ErrorResponse = response.json().await?;
                let msg =
                    format!("Error while getting campaign with id {campaign_id} --> \n{error:#?}");
                return Err(msg.into());
            }
        }
    }
    /// Возвращает список магазинов, к которым у пользователя с указанным логином есть доступ.
    pub async fn get_logins_campaigns(&self, login: impl Into<String>) -> Result<Vec<CampaignDTO>> {
        let uri = format!("{}campaigns/by_login/{}", crate::BASE_URL, login.into());
        debug!("Getting login's campaigns");
        let result = self.get_campaigns_with_uri(uri).await?;
        Ok(result)
    }
    /// Возвращает информацию о настройках магазина, идентификатор которого указан в запросе.
    #[instrument]
    pub async fn get_settings(&self, campaign_id: i64) -> Result<CampaignSettingsDTO> {
        let uri = format!("{}campaigns/{campaign_id}/settings", crate::BASE_URL);
        debug!("Getting campaign with id: {campaign_id} settings");
        let response = self
            .api_client
            .client()
            .get(&uri)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let settings: SettingsResponse = response.json().await?;
                Ok(settings.settings)
            }
            _ => {
                let error: ErrorResponse = response.json().await?;
                let msg =
                    format!("Error while getting campaign with id {campaign_id} --> \n{error:#?}");
                return Err(msg.into());
            }
        }
    }
    async fn get_campaigns_with_uri(&self, uri: impl Into<String>) -> Result<Vec<CampaignDTO>> {
        let uri: String = uri.into();
        let mut page = 1;
        let mut result = Vec::new();
        loop {
            let response = self
                .api_client
                .client()
                .get(&uri)
                .query(&[("page", page)])
                .bearer_auth(self.api_client.access_token())
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let campaigns_response: CampaignsResponse = response.json().await?;
                    result.extend(campaigns_response.campaigns);
                    if campaigns_response
                        .pager
                        .pages_count
                        .is_some_and(|p| p > page)
                    {
                        page += 1;
                    } else {
                        break;
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg =
                        format!("Error while getting campaigns with page {page} --> \n{error:#?}");
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
}
impl MarketClient {
    /// Магазины    
    ///
    /// # Example
    ///
    /// ```rust
    ///use rust_yandexmarket::{MarketClient, Result};
    ///
    ///#[tokio::main]
    ///async fn main() -> Result<()> {
    ///    tracing_subscriber::fmt::init();
    ///    let client = MarketClient::init().await?;
    ///    let campaigns = client.campaigns().get_all_campaigns().await?;
    ///    let id = campaigns.first().unwrap().id;
    ///    let _campaign = client.campaigns().get_campaign(id).await?;
    ///    let logins = client.campaigns().get_logins(id).await?;
    ///    let login = logins.first().unwrap();
    ///    let _login_campaigns = client.campaigns().get_logins_campaigns(login).await?;
    ///    let settings = client.campaigns().get_settings(id).await?;
    ///    println!("{settings:#?}");
    ///    Ok(())
    ///}
    /// ```
    pub fn campaigns(&self) -> Campaigns {
        Campaigns { api_client: self }
    }
}
#[cfg(test)]
mod tests {
    use crate::{MarketClient, Result};
    #[tokio::test]
    async fn test_get_all_campaigns() -> Result<()> {
        let client = MarketClient::init().await?;
        let campaigns = client.campaigns().get_all_campaigns().await?;
        assert!(!campaigns.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_campaign() -> Result<()> {
        let client = MarketClient::init().await?;
        let campaign = client
            .campaigns()
            .get_campaign(client.campaign_id())
            .await?;
        assert_eq!(campaign.id, client.campaign_id());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_logins() -> Result<()> {
        let client = MarketClient::init().await?;
        let logins = client.campaigns().get_logins(client.campaign_id()).await?;
        assert!(!logins.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_logins_campaigns() -> Result<()> {
        let client = MarketClient::init().await?;
        let login = client
            .campaigns()
            .get_logins(client.campaign_id())
            .await?
            .first()
            .unwrap()
            .to_string();
        let campaigns = client.campaigns().get_logins_campaigns(login).await?;
        assert!(!campaigns.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_campaign_settings() -> Result<()> {
        let client = MarketClient::init().await?;
        let settings = client
            .campaigns()
            .get_settings(client.campaign_id())
            .await?;
        assert!(settings.show_in_premium);
        Ok(())
    }
}
