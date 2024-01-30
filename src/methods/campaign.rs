use tracing::{debug, instrument};

use crate::{
    models::campaigns::{CampaignDTO, CampaignsResponse},
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
            let campaigns_response: CampaignsResponse = self
                .client()
                .get(&campaigns_uri)
                .query(&[("page", page)])
                .bearer_auth(self.access_token())
                .send()
                .await?
                .json()
                .await?;
            result.extend(campaigns_response.campaigns);
            if campaigns_response.pager.pages_count > page {
                page += 1;
            } else {
                break;
            }
        }
        Ok(result)
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
}
