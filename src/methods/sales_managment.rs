use crate::{
    models::{
        sales_managment::{
            StockRequest, StockResponse, UpdateCampaignOffersRequest, UpdateStockRequest,
            WarehouseOffersDTO,
        },
        ErrorResponse,
    },
    MarketClient, Result, StockDTO, UpdateCampaignOfferDTO,
};

#[derive(Debug, Clone)]
pub struct SalesManagment<'a> {
    api_client: &'a MarketClient,
}
impl MarketClient {
    /// Управление продажами
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result, UpdateCampaignOfferDTO};
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let acc = UpdateCampaignOfferDTO::builder()
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .available(false)
    ///         .min_quantity(5)
    ///         .step_quantity(1)
    ///         .vat(6)
    ///         .build();
    ///     client.sales_managment().update_offers(vec![acc]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn sales_managment(&self) -> SalesManagment {
        SalesManagment { api_client: self }
    }
}
impl<'a> SalesManagment<'a> {
    // pub async fn get_offers(&self) -> Result<Vec<GetCampaignOfferDTO>> {
    //     let mut result = Vec::new();
    //     let mut next_page_token = None;
    //     let req = OffersRequest {
    //         offer_ids: Some(vec![]),
    //         statuses: Some(vec![]),
    //         category_ids: Some(vec![]),
    //         vendor_names: Some(vec![]),
    //         tags: Some(vec![]),
    //     };
    //     loop {
    //         let uri = match next_page_token {
    //             Some(s) => format!(
    //                 "{}campaigns/{}/offers?page_token={}",
    //                 crate::BASE_URL,
    //                 self.api_client.campaign_id(),
    //                 s
    //             ),
    //             None => format!(
    //                 "{}campaigns/{}/offers",
    //                 crate::BASE_URL,
    //                 self.api_client.campaign_id()
    //             ),
    //         };
    //         let response = self
    //             .api_client
    //             .client()
    //             .get(&uri)
    //             .bearer_auth(self.api_client.access_token())
    //             .json(&req)
    //             .send()
    //             .await?;
    //         match response.status() {
    //             reqwest::StatusCode::OK => {
    //                 let resp: OffersResponse = response.json().await?;
    //                 let Some(res) = resp.result else {
    //                     break;
    //                 };
    //                 if let Some(offers) = res.offers {
    //                     result.extend(offers);
    //                 } else {
    //                     break;
    //                 }
    //                 if let Some(paging) = res.paging {
    //                     if paging.next_page_token.is_some() {
    //                         next_page_token = paging.next_page_token;
    //                     } else {
    //                         break;
    //                     }
    //                 } else {
    //                     break;
    //                 }
    //             }
    //             _ => {
    //                 let err_resp: ErrorResponse = response.json().await?;
    //                 let msg = format!("{err_resp:#?}");
    //                 return Err(msg.into());
    //             }
    //         }
    //     }
    //     Ok(result)
    // }
    /// Настройка размещения товаров в магазине
    /// Изменяет параметры размещения товаров в конкретном магазине: доступность товара, условия доставки и самовывоза, применяемую ставку НДС.
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result, UpdateCampaignOfferDTO};
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let acc = UpdateCampaignOfferDTO::builder()
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .available(false)
    ///         .min_quantity(5)
    ///         .step_quantity(1)
    ///         .vat(6)
    ///         .build();
    ///     client.sales_managment().update_offers(vec![acc]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_offers(&self, offers: Vec<UpdateCampaignOfferDTO>) -> Result<()> {
        let uri = format!(
            "{}campaigns/{}/offers/update",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let mut errors = String::new();
        for offers_chunk in offers.chunks(500) {
            let req = UpdateCampaignOffersRequest {
                offers: offers_chunk.to_vec(),
            };
            let response = self
                .api_client
                .client()
                .post(&uri)
                .json(&req)
                .bearer_auth(self.api_client.access_token())
                .send()
                .await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
            match response.status() {
                reqwest::StatusCode::OK => continue,
                _ => {
                    let err_resp: ErrorResponse = response.json().await?;
                    let msg = format!("{err_resp:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into())
        }
    }
    /// Передает данные об остатках товаров на витрине.
    ///
    /// Обязательно указывайте SKU в точности так, как он указан в каталоге. Например, 557722 и 0557722 — это два разных SKU.
    ///
    /// Данные в каталоге обновляются не мгновенно
    ///
    /// Это занимает до нескольких минут.
    ///
    /// ⚙️ Лимит: 100 000 товаров в минуту, не более 500 товаров в одном запросе с 1 марта 2024 года
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result, StockDTO, UpdateCampaignOfferDTO};
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let offer_id = "Homakoll_164_Prof_1.3";
    ///     let stock = vec![StockDTO::builder()
    ///         .sku(offer_id)
    ///         .warehouse_id(78079)
    ///         .count(6)
    ///         .count(3)
    ///         .build()];
    ///     client.sales_managment().stock_update(stock).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn stock_update(&self, stock: Vec<StockDTO>) -> Result<()> {
        let uri = format!(
            "{}campaigns/{}/offers/stocks",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let mut errors = String::new();
        for items in stock.chunks(500) {
            let req = UpdateStockRequest {
                skus: items.to_vec(),
            };
            let response = self
                .api_client
                .client()
                .put(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&req)
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => continue,
                _ => {
                    let err_resp: ErrorResponse = response.json().await?;
                    let msg = format!("{err_resp:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into())
        }
    }
    /// Информация об остатках и оборачиваемости
    /// Возвращает данные об остатках товаров (для моделей FBY, FBS и Экспресс) и об оборачиваемости товаров (для модели FBY).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result, StockDTO, UpdateCampaignOfferDTO};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let offer_id = "Homakoll_164_Prof_1.3";
    ///     let stock = client.sales_managment().retrieve_stock().await?;
    ///     dbg!(stock);
    ///     let filtered_stock = client
    ///         .sales_managment()
    ///         .retrieve_stock_with_ids(vec![offer_id.to_string()])
    ///         .await?;
    ///     dbg!(filtered_stock);
    ///     Ok(())
    /// }
    /// ```
    pub async fn retrieve_stock(&self) -> Result<Vec<WarehouseOffersDTO>> {
        let req = StockRequest {
            with_turnover: Some(true),
            archived: None,
            offer_ids: None,
        };
        let mut result = Vec::new();
        let mut next_page_token = None;
        loop {
            let uri = match next_page_token {
                Some(s) => {
                    format!(
                        "{}campaigns/{}/offers/stocks?page_token={s}",
                        crate::BASE_URL,
                        self.api_client.campaign_id()
                    )
                }
                None => {
                    format!(
                        "{}campaigns/{}/offers/stocks",
                        crate::BASE_URL,
                        self.api_client.campaign_id()
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
                    let warehouse_response: StockResponse = response.json().await?;
                    let Some(response_result) = warehouse_response.result else {
                        break;
                    };
                    let Some(stock) = response_result.warehouses else {
                        break;
                    };
                    if stock.is_empty() {
                        break;
                    } else {
                        result.extend(stock);
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
                    let r: serde_json::Value = response.json().await?;
                    let msg = format!("{r:#?}");
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
    /// Информация об остатках и оборачиваемости
    /// Возвращает данные об остатках товаров (для моделей FBY, FBS и Экспресс) и об оборачиваемости товаров (для модели FBY).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result, StockDTO, UpdateCampaignOfferDTO};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let offer_id = "Homakoll_164_Prof_1.3";
    ///     let stock = client.sales_managment().retrieve_stock().await?;
    ///     dbg!(stock);
    ///     let filtered_stock = client
    ///         .sales_managment()
    ///         .retrieve_stock_with_ids(vec![offer_id.to_string()])
    ///         .await?;
    ///     dbg!(filtered_stock);
    ///     Ok(())
    /// }
    /// ```
    pub async fn retrieve_stock_with_ids(
        &self,
        offer_ids: Vec<String>,
    ) -> Result<Vec<WarehouseOffersDTO>> {
        let req = StockRequest {
            with_turnover: Some(true),
            archived: None,
            offer_ids: Some(offer_ids),
        };
        let uri = format!(
            "{}campaigns/{}/offers/stocks",
            crate::BASE_URL,
            self.api_client.campaign_id()
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
            reqwest::StatusCode::OK => {
                let warehouse_response: StockResponse = response.json().await?;
                if let Some(response_result) = warehouse_response.result {
                    if let Some(stock) = response_result.warehouses {
                        Ok(stock)
                    } else {
                        Ok(vec![])
                    }
                } else {
                    Ok(vec![])
                }
            }
            _ => {
                let er_res: ErrorResponse = response.json().await?;
                let msg = format!("Error: {er_res:#?}");
                Err(msg.into())
            }
        }
    }
}
