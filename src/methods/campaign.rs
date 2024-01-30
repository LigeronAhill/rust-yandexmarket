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
impl MarketClient {
    /// Возвращает список магазинов, к которым имеет доступ пользователь — владелец
    /// авторизационного токена, использованного в запросе. Для агентских пользователей
    /// список состоит из подагентских магазинов.
    #[instrument]
    pub async fn get_all_campaigns(&self) -> Result<Vec<CampaignDTO>> {
        let campaigns_uri = format!("{}campaigns", self.base_url());
        debug!("Getting campaigns");
        let mut page = 1;
        let mut result = Vec::new();
        loop {
            let response = self
                .client()
                .get(&campaigns_uri)
                .query(&[("page", page)])
                .bearer_auth(self.access_token())
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let campaigns_response: CampaignsResponse = response.json().await?;
                    result.extend(campaigns_response.campaigns);
                    if campaigns_response.pager.pages_count > page {
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
    /// Информация о магазине с заданным id
    #[instrument]
    pub async fn get_campaign(&self, campaign_id: i64) -> Result<CampaignDTO> {
        let uri = format!("{}campaigns/{campaign_id}", self.base_url());
        debug!("Getting campaign with id: {campaign_id}");
        let response = self
            .client()
            .get(&uri)
            .bearer_auth(self.access_token())
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
        let uri = format!("{}campaigns/{campaign_id}/logins", self.base_url());
        debug!("Getting campaign with id: {campaign_id} logins");
        let response = self
            .client()
            .get(&uri)
            .bearer_auth(self.access_token())
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
        let uri = format!("{}campaigns/by_login/{}", self.base_url(), login.into());
        debug!("Getting login's campaigns");
        let mut page = 1;
        let mut result = Vec::new();
        loop {
            let response = self
                .client()
                .get(&uri)
                .query(&[("page", page)])
                .bearer_auth(self.access_token())
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let campaigns_response: CampaignsResponse = response.json().await?;
                    result.extend(campaigns_response.campaigns);
                    if campaigns_response.pager.pages_count > page {
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
    /// Возвращает информацию о настройках магазина, идентификатор которого указан в запросе.
    #[instrument]
    pub async fn get_settings(&self, campaign_id: i64) -> Result<CampaignSettingsDTO> {
        let uri = format!("{}campaigns/{campaign_id}/settings", self.base_url());
        debug!("Getting campaign with id: {campaign_id} settings");
        let response = self
            .client()
            .get(&uri)
            .bearer_auth(self.access_token())
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
}
#[cfg(test)]
mod tests {
    use crate::{MarketClient, Result};
    #[tokio::test]
    async fn test_get_all_campaigns() -> Result<()> {
        let client = MarketClient::init().await?;
        let campaigns = client.get_all_campaigns().await?;
        assert!(!campaigns.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_campaign() -> Result<()> {
        let client = MarketClient::init().await?;
        let id = client.campaigns().first().unwrap().id;
        let campaign = client.get_campaign(id).await?;
        assert_eq!(campaign.id, id);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_logins() -> Result<()> {
        let client = MarketClient::init().await?;
        let id = client.campaigns().first().unwrap().id;
        let logins = client.get_logins(id).await?;
        assert!(!logins.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_logins_campaigns() -> Result<()> {
        let client = MarketClient::init().await?;
        let id = client.campaigns().first().unwrap().id;
        let login = client.get_logins(id).await?.first().unwrap().to_string();
        let campaigns = client.get_logins_campaigns(login).await?;
        assert!(!campaigns.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_campaign_settings() -> Result<()> {
        let client = MarketClient::init().await?;
        let id = client.campaigns().first().unwrap().id;
        let settings = client.get_settings(id).await?;
        assert!(settings.show_in_premium);
        Ok(())
    }
}
