use std::vec;

use crate::{
    models::{
        sales_managment::{
            GetCampaignOfferDTO, OffersRequest, OffersResponse, UpdateCampaignOffersRequest,
        },
        ErrorResponse,
    },
    MarketClient, Result, UpdateCampaignOfferDTO,
};

#[derive(Debug, Clone)]
pub struct SalesManagment<'a> {
    api_client: &'a MarketClient,
}
impl MarketClient {
    pub fn sales_managment(&self) -> SalesManagment {
        SalesManagment { api_client: self }
    }
}
impl<'a> SalesManagment<'a> {
    pub async fn get_offers(&self) -> Result<Vec<GetCampaignOfferDTO>> {
        let mut result = Vec::new();
        let mut next_page_token = None;
        let req = OffersRequest {
            offer_ids: Some(vec![]),
            statuses: Some(vec![]),
            category_ids: Some(vec![]),
            vendor_names: Some(vec![]),
            tags: Some(vec![]),
        };
        loop {
            let uri = match next_page_token {
                Some(s) => format!(
                    "{}campaigns/{}/offers?page_token={}",
                    crate::BASE_URL,
                    self.api_client.campaign_id(),
                    s
                ),
                None => format!(
                    "{}campaigns/{}/offers",
                    crate::BASE_URL,
                    self.api_client.campaign_id()
                ),
            };
            let response = self
                .api_client
                .client()
                .get(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&req)
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let resp: OffersResponse = response.json().await?;
                    let Some(res) = resp.result else {
                        break;
                    };
                    if let Some(offers) = res.offers {
                        result.extend(offers);
                    } else {
                        break;
                    }
                    if let Some(paging) = res.paging {
                        if paging.next_page_token.is_some() {
                            next_page_token = paging.next_page_token;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    let err_resp: ErrorResponse = response.json().await?;
                    let msg = format!("{err_resp:#?}");
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
    pub async fn update_offers(&self, offers: Vec<UpdateCampaignOfferDTO>) -> Result<()> {
        let req = UpdateCampaignOffersRequest { offers };
        let uri = format!(
            "{}campaigns/{}/offers/update",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let response = self
            .api_client
            .client()
            .post(&uri)
            .json(&req)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                let err_resp: ErrorResponse = response.json().await?;
                let msg = format!("{err_resp:#?}");
                Err(msg.into())
            }
        }
    }
}
