use crate::models::offer_cards::{OfferCardDTO, OfferCardResponse};
use crate::models::ErrorResponse;
use crate::{MarketClient, Result};

#[derive(Debug, Clone)]
pub struct OfferCards<'a> {
    api_client: &'a MarketClient,
}
impl MarketClient {
    pub fn offer_cards(&self) -> OfferCards {
        OfferCards { api_client: self }
    }
}

impl<'a> OfferCards<'a> {
    /// Возвращает сведения о состоянии контента для заданных товаров:
    /// создана ли карточка товара и в каком она статусе;
    /// заполненность карточки в процентах;
    /// есть ли ошибки или предупреждения, связанные с контентом;
    /// рекомендации по заполнению карточки.
    /// ⚙️ Лимит: рассчитывается по формуле суточный лимит товаров — количество товаров в каталоге * 25
    pub async fn get_all_offer_cards(&self) -> Result<Vec<OfferCardDTO>> {
        let mut result = Vec::new();
        let mut next_page_token = None;
        loop {
            let uri = match next_page_token {
                Some(s) => {
                    format!(
                        "{}businesses/{}/offer-cards?page_token={s}",
                        crate::BASE_URL,
                        self.api_client.business_id()
                    )
                }
                None => {
                    format!(
                        "{}businesses/{}/offer-cards",
                        crate::BASE_URL,
                        self.api_client.business_id()
                    )
                }
            };
            let response = self
                .api_client
                .client()
                .post(&uri)
                .bearer_auth(self.api_client.access_token())
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let offer_cards_response: OfferCardResponse = response.json().await?;
                    let Some(response_result) = offer_cards_response.result else {
                        break;
                    };
                    let Some(offers) = response_result.offer_cards else {
                        break;
                    };
                    if offers.is_empty() {
                        break;
                    } else {
                        result.extend(offers);
                    }
                    if let Some(pager) = response_result.paging {
                        if pager.next_page_token.is_none() {
                            break;
                        } else {
                            next_page_token = pager.next_page_token;
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    let er_res: ErrorResponse = response.json().await?;
                    let msg = format!("Error: {er_res:#?}");
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
}
