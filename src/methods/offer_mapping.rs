use tracing::{debug, instrument};

use crate::{
    models::{
        offer_mappings::{GetOfferMappingDTO, OfferMappingResponse},
        ErrorResponse,
    },
    MarketClient, Result,
};

impl MarketClient {
    pub fn offer_mappings(&self) -> OfferMapping {
        OfferMapping { api_client: self }
    }
}
#[derive(Debug, Clone)]
pub struct OfferMapping<'a> {
    api_client: &'a MarketClient,
}
impl<'a> OfferMapping<'a> {
    /// Возвращает список товаров в каталоге с параметрами каждого товара.
    ///
    /// Можно использовать тремя способами:
    ///
    /// задать список интересующих SKU;
    /// задать фильтр — в этом случае результаты возвращаются постранично;
    /// не передавать тело запроса, чтобы получить список всех товаров в каталоге.
    /// ⚙️ Лимит: 600 запросов в минуту, не более 200 товаров в одном запросе
    #[instrument]
    pub async fn get_all_offers(&self) -> Result<Vec<GetOfferMappingDTO>> {
        debug!("Getting offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut next_page_token = None;
        let mut result = Vec::new();
        loop {
            let response = match next_page_token {
                Some(page_token) => {
                    self.api_client
                        .client()
                        .post(&uri)
                        .query(&[("page_token", page_token)])
                        .bearer_auth(self.api_client.access_token())
                        .send()
                        .await?
                }
                None => {
                    self.api_client
                        .client()
                        .post(&uri)
                        .bearer_auth(self.api_client.access_token())
                        .send()
                        .await?
                }
            };
            match response.status() {
                reqwest::StatusCode::OK => {
                    let offers_response: OfferMappingResponse = response.json().await?;
                    result.extend(offers_response.result.offer_mappings);
                    next_page_token = offers_response.result.paging.next_page_token;
                    if next_page_token.is_none() {
                        break;
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while getting offer-mappings\n{error:#?}");
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
}
