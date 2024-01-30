use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use crate::{
    models::campaigns::{CampaignDTO, CampaignResponse, CampaignsResponse},
    MarketClient, Result,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ApiResponseStatusType {
    OK,
    ERROR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiErrorDTO {
    code: String,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorResponse {
    status: Option<ApiResponseStatusType>,
    error: Option<Vec<ApiErrorDTO>>,
}
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
    /// Информация о магазине
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
}
#[cfg(test)]
mod tests {
    use crate::{MarketClient, Result};
    #[tokio::test]
    async fn test_get_all_campaigns() -> Result<()> {
        // -- Setup & Fixtures
        let client = MarketClient::init().await?;
        // -- Exec
        let campaigns = client.get_all_campaigns().await?;
        // -- Check
        assert!(!campaigns.is_empty());
        // -- Clean

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
}
