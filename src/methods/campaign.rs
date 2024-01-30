use crate::{
    models::campaigns::{CampaignDTO, CampaignsResponse},
    MarketClient, Result,
};

impl MarketClient {
    pub async fn get_all_campaigns(&self) -> Result<Vec<CampaignDTO>> {
        let campaigns_uri = format!("{}campaigns", self.base_url());
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
