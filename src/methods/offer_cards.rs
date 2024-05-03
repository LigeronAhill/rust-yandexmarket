use crate::models::offer_cards::{
    CategoryCharacteristicsResponse, CategoryParameterDTO, OfferCardDTO, OfferCardResponse,
    UpdateOfferCardsRequest,
};
use crate::models::ErrorResponse;
use crate::{MarketClient, OfferCardRequest, OfferContentDTO};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct OfferCards<'a> {
    api_client: &'a MarketClient,
}
impl MarketClient {
    /// Возвращает методы для работы с карточками товаров
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, OfferCardRequest, OfferContentDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let market_category_id = 15317135;
    ///     let cards = client.offer_cards().get_all_offer_cards().await?;
    ///     dbg!(&cards);
    ///     let req = OfferCardRequest::builder()
    ///         .category_id(market_category_id)
    ///         .build();
    ///     let filtered = client
    ///         .offer_cards()
    ///         .get_all_offer_cards_with_filter(req)
    ///         .await?;
    ///     dbg!(&filtered);
    ///     let chars = client
    ///         .offer_cards()
    ///         .get_category_characteristics(market_category_id)
    ///         .await?;
    ///     dbg!(&chars);
    ///     let card = OfferContentDTO::builder()
    ///         .category_id(market_category_id)
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .parameter_value(15366467, Some(29602430), "ведро")
    ///         .build();
    ///     let _ = client.offer_cards().update_offer_cards(vec![card]).await?;
    ///     Ok(())
    /// }
    /// ```
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
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, OfferCardRequest, OfferContentDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let market_category_id = 15317135;
    ///     let cards = client.offer_cards().get_all_offer_cards().await?;
    ///     dbg!(&cards);
    ///     let req = OfferCardRequest::builder()
    ///         .category_id(market_category_id)
    ///         .build();
    ///     let filtered = client
    ///         .offer_cards()
    ///         .get_all_offer_cards_with_filter(req)
    ///         .await?;
    ///     dbg!(&filtered);
    ///     let chars = client
    ///         .offer_cards()
    ///         .get_category_characteristics(market_category_id)
    ///         .await?;
    ///     dbg!(&chars);
    ///     let card = OfferContentDTO::builder()
    ///         .category_id(market_category_id)
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .parameter_value(15366467, Some(29602430), "ведро")
    ///         .build();
    ///     let _ = client.offer_cards().update_offer_cards(vec![card]).await?;
    ///     Ok(())
    /// }
    /// ```
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
                    return Err(anyhow!(msg));
                }
            }
        }
        Ok(result)
    }
    /// Возвращает сведения о состоянии контента для заданных товаров:
    /// создана ли карточка товара и в каком она статусе;
    /// заполненность карточки в процентах;
    /// есть ли ошибки или предупреждения, связанные с контентом;
    /// рекомендации по заполнению карточки.
    /// ⚙️ Лимит: рассчитывается по формуле суточный лимит товаров — количество товаров в каталоге * 25
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, OfferCardRequest, OfferContentDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let market_category_id = 15317135;
    ///     let cards = client.offer_cards().get_all_offer_cards().await?;
    ///     dbg!(&cards);
    ///     let req = OfferCardRequest::builder()
    ///         .category_id(market_category_id)
    ///         .build();
    ///     let filtered = client
    ///         .offer_cards()
    ///         .get_all_offer_cards_with_filter(req)
    ///         .await?;
    ///     dbg!(&filtered);
    ///     let chars = client
    ///         .offer_cards()
    ///         .get_category_characteristics(market_category_id)
    ///         .await?;
    ///     dbg!(&chars);
    ///     let card = OfferContentDTO::builder()
    ///         .category_id(market_category_id)
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .parameter_value(15366467, Some(29602430), "ведро")
    ///         .build();
    ///     let _ = client.offer_cards().update_offer_cards(vec![card]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_all_offer_cards_with_filter(
        &self,
        req: OfferCardRequest,
    ) -> Result<Vec<OfferCardDTO>> {
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
                .json(&req)
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
                    return Err(anyhow!(msg));
                }
            }
        }
        Ok(result)
    }
    /// Возвращает список характеристик с допустимыми значениями для заданной категории.
    ///
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, OfferCardRequest, OfferContentDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let market_category_id = 15317135;
    ///     let cards = client.offer_cards().get_all_offer_cards().await?;
    ///     dbg!(&cards);
    ///     let req = OfferCardRequest::builder()
    ///         .category_id(market_category_id)
    ///         .build();
    ///     let filtered = client
    ///         .offer_cards()
    ///         .get_all_offer_cards_with_filter(req)
    ///         .await?;
    ///     dbg!(&filtered);
    ///     let chars = client
    ///         .offer_cards()
    ///         .get_category_characteristics(market_category_id)
    ///         .await?;
    ///     dbg!(&chars);
    ///     let card = OfferContentDTO::builder()
    ///         .category_id(market_category_id)
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .parameter_value(15366467, Some(29602430), "ведро")
    ///         .build();
    ///     let _ = client.offer_cards().update_offer_cards(vec![card]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_category_characteristics(
        &self,
        category_id: i64,
    ) -> Result<Vec<CategoryParameterDTO>> {
        let uri = format!("{}category/{category_id}/parameters", crate::BASE_URL);
        let mut result = Vec::new();
        let response = self
            .api_client
            .client()
            .post(&uri)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let chars_resp: CategoryCharacteristicsResponse = response.json().await?;
                if let Some(res) = chars_resp.result {
                    if let Some(params) = res.parameters {
                        result.extend(params);
                    }
                }
            }
            _ => {
                let er_res: ErrorResponse = response.json().await?;
                let msg = format!("Error: {er_res:#?}");
                return Err(anyhow!(msg));
            }
        }
        Ok(result)
    }
    /// Редактирует характеристики товара, которые специфичны для категории, к которой он относится.
    /// Всегда передавайте список характеристик и значений целиком, даже если нужно изменить всего одну из них. Передача отсутствующего значения стирает значение, сохраненное ранее.
    ///
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, OfferCardRequest, OfferContentDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let market_category_id = 15317135;
    ///     let cards = client.offer_cards().get_all_offer_cards().await?;
    ///     dbg!(&cards);
    ///     let req = OfferCardRequest::builder()
    ///         .category_id(market_category_id)
    ///         .build();
    ///     let filtered = client
    ///         .offer_cards()
    ///         .get_all_offer_cards_with_filter(req)
    ///         .await?;
    ///     dbg!(&filtered);
    ///     let chars = client
    ///         .offer_cards()
    ///         .get_category_characteristics(market_category_id)
    ///         .await?;
    ///     dbg!(&chars);
    ///     let card = OfferContentDTO::builder()
    ///         .category_id(market_category_id)
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .parameter_value(15366467, Some(29602430), "ведро")
    ///         .build();
    ///     let _ = client.offer_cards().update_offer_cards(vec![card]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_offer_cards(&self, cards: Vec<OfferContentDTO>) -> Result<()> {
        let req = UpdateOfferCardsRequest {
            offers_content: cards,
        };
        let uri = format!(
            "{}businesses/{}/offer-cards/update",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let response = self
            .api_client
            .client()
            .post(&uri)
            .bearer_auth(self.api_client.access_token())
            .json(&req)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                let er_res: ErrorResponse = response.json().await?;
                let msg = format!("Error: {er_res:#?}");
                Err(anyhow!(msg))
            }
        }
    }
}
