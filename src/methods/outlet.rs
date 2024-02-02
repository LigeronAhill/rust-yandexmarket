use tracing::{debug, instrument};

use crate::{
    models::{
        outlets::{
            CreateOutletDTO, CreateOutletResponse, FullOutletDTO, FullOutletLicenseDTO,
            LicenseResponse, OutletResponse, OutletsResponse,
        },
        ErrorResponse,
    },
    MarketClient, Result,
};
#[derive(Debug, Clone)]
pub struct Outlets<'a> {
    api_client: &'a MarketClient,
}
impl<'a> Outlets<'a> {
    /// Возвращает список точек продаж магазина.
    ///
    /// В течение суток этим и другими запросами о точках продаж, кроме запроса GET delivery/services, можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.
    ///
    /// ⚙️ Лимит: 100 000 запросов в час
    ///
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     // let outlets = client.outlets().get_all_outlets().await?;
    ///     // do something...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get_all_outlets(&self) -> Result<Vec<FullOutletDTO>> {
        let mut result = Vec::new();
        let mut next_page_token = None;
        loop {
            let uri = match next_page_token {
                Some(s) => {
                    format!(
                        "{}campaigns/{}/outlets?nextPageToken={s}",
                        crate::BASE_URL,
                        self.api_client.campaign_id()
                    )
                }
                None => {
                    format!(
                        "{}campaigns/{}/outlets",
                        crate::BASE_URL,
                        self.api_client.campaign_id()
                    )
                }
            };
            let response = self
                .api_client
                .client()
                .get(&uri)
                .bearer_auth(self.api_client.access_token())
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let outlets_response: OutletsResponse = response.json().await?;
                    result.extend(outlets_response.outlets);
                    if outlets_response.paging.next_page_token.is_some() {
                        next_page_token = outlets_response.paging.next_page_token;
                    } else {
                        break;
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg =
                        format!("Error while getting outlets with uri {uri} --> \n{error:#?}");
                    return Err(msg.into());
                }
            }
        }
        debug!("Getting outlets");
        Ok(result)
    }
    /// Возвращает информацию о точках продаж магазина.
    ///
    /// В течение суток этим и другими запросами о точках продаж, кроме запроса GET delivery/services, можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.
    ///
    /// ⚙️ Лимит: 100 000 запросов в час
    ///
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     // let outlet = client.outlets().get_outlet(outlet_id).await?;
    ///     // do something...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get_outlet(&self, outlet_id: i64) -> Result<FullOutletDTO> {
        let uri = format!(
            "{}campaigns/{}/outlets/{outlet_id}",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        debug!("Getting outlet with id {outlet_id}");
        let response = self
            .api_client
            .client()
            .get(&uri)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let outlet: OutletResponse = response.json().await?;
                Ok(outlet.outlet)
            }
            _ => {
                let error: ErrorResponse = response.json().await?;
                let msg =
                    format!("Error while getting outlet with id {outlet_id} --> \n{error:#?}");
                return Err(msg.into());
            }
        }
    }
    /// Создает точку продаж магазина на Маркете.
    ///
    /// В течение суток этим и другими запросами о точках продаж, кроме запроса GET delivery/services, можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.
    ///
    /// ⚙️ Лимит: 100 000 запросов в час
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let address = rust_yandexmarket::Address::builder()
    ///         .region_id(13)
    ///         .street("улица Ленина")
    ///         .number("69")
    ///         .block("5")
    ///         .additional("Вход со двора")
    ///         .build();
    ///     let schedule_item_1 = rust_yandexmarket::WorkingScheduleItem::builder()
    ///         .start_day(rust_yandexmarket::DayOfWeekType::Monday)
    ///         .end_day(rust_yandexmarket::DayOfWeekType::Friday)
    ///         .start_time("09:00")
    ///         .end_time("21:00")
    ///         .build();
    ///     let schedule_item_2 = rust_yandexmarket::WorkingScheduleItem::builder()
    ///         .start_day(rust_yandexmarket::DayOfWeekType::Saturday)
    ///         .end_day(rust_yandexmarket::DayOfWeekType::Sunday)
    ///         .start_time("10:00")
    ///         .end_time("18:00")
    ///         .build();
    ///     let delivery_rule = rust_yandexmarket::DeliveryRule::builder()
    ///         .cost(0)
    ///         .min_delivery_days(5)
    ///         .max_delivery_days(7)
    ///         .order_before(15)
    ///         .build()?;
    ///     let outlet_to_create = rust_yandexmarket::Outlet::builder()
    ///         .name("Test Outlet")
    ///         .outlet_type(rust_yandexmarket::OutletType::Retail)
    ///         .coords("20.45, 54.71")
    ///         .is_main(false)
    ///         .shop_outlet_code("42")
    ///         .visibility(rust_yandexmarket::OutletVisibilityType::Hidden)
    ///         .address(address)
    ///         .phone("+7 (999) 696-69-69")
    ///         .phone("+7 (888) 999-66-99")
    ///         .phones(vec![
    ///             "+7 (678) 321-65-49".to_string(),
    ///             "+7 (987) 654-32-11".to_string(),
    ///         ])
    ///         .work_in_holiday(true)
    ///         .schedule_item(schedule_item_1)
    ///         .schedule_item(schedule_item_2)
    ///         .delivery_rule(delivery_rule)
    ///         .email("most@wanted.man")
    ///         .storage_period(3)
    ///         .build();
    ///     // let created = client.outlets().create_outlet(outlet_to_create).await?;
    ///     // do something...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn create_outlet(&self, outlet: CreateOutletDTO) -> Result<i64> {
        debug!("Creating outlet");
        let uri = format!(
            "{}campaigns/{}/outlets",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let response = self
            .api_client
            .client()
            .post(&uri)
            .bearer_auth(self.api_client.access_token())
            .json(&outlet)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let res: CreateOutletResponse = response.json().await?;
                Ok(res.result.id)
            }
            _ => {
                let err: ErrorResponse = response.json().await?;
                let msg = format!("Error creating outlet\n{err:#?}");
                Err(msg.into())
            }
        }
    }
    /// Изменяет информацию о точке продаж магазина на Маркете.
    ///
    /// В течение суток этим и другими запросами о точках продаж, кроме запроса GET delivery/services, можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.
    ///
    /// ⚙️ Лимит: 100 000 запросов в час
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let outlets = client.outlets().get_all_outlets().await?;
    ///     let mut outlet_to_update = outlets.first().unwrap();
    ///     outlet_to_update.name = "Another name".to_string();
    ///     // client
    ///     //     .outlets()
    ///     //     .update_outlet(created, outlet_to_update)
    ///     //     .await?;
    ///     // do something...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn update_outlet(&self, outlet_id: i64, outlet: FullOutletDTO) -> Result<()> {
        debug!("Updating outlet");
        let uri = format!(
            "{}campaigns/{}/outlets/{outlet_id}",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let response = self
            .api_client
            .client()
            .put(&uri)
            .bearer_auth(self.api_client.access_token())
            .json(&outlet)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                let err: ErrorResponse = response.json().await?;
                let msg = format!("Error updating outlet\n{err:#?}");
                Err(msg.into())
            }
        }
    }
    /// Удаляет точку продаж магазина на Маркете.
    ///
    /// В течение суток этим и другими запросами о точках продаж, кроме запроса GET delivery/services, можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.
    ///
    /// ⚙️ Лимит: 100 000 запросов в час
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     // client.outlets().delete_outlet(outlet_id).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn delete_outlet(&self, outlet_id: i64) -> Result<()> {
        debug!("Deleting outlet");
        let uri = format!(
            "{}/campaigns/{}/outlets/{outlet_id}",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let response = self
            .api_client
            .client()
            .delete(&uri)
            .bearer_auth(self.api_client.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                let err: ErrorResponse = response.json().await?;
                let msg = format!("Error deleting outlet\n{err:#?}");
                Err(msg.into())
            }
        }
    }
    /// Возвращает информацию о лицензиях для точек продаж.
    ///
    /// В течение суток этим и другими запросами о точках продаж, кроме запроса GET delivery/services, можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.
    ///
    /// ⚙️ Лимит: 100 000 запросов в час
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MarketClient::init().await?;
    ///     let licenses = client.outlets().get_all_licenses(357750157).await?;
    ///     // do something...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get_all_licenses(&self, outlet_id: i64) -> Result<Vec<FullOutletLicenseDTO>> {
        debug!("Getting licenses");
        let uri = format!(
            "{}campaigns/{}/outlets/licenses",
            crate::BASE_URL,
            self.api_client.campaign_id()
        );
        let response = self
            .api_client
            .client()
            .get(&uri)
            .bearer_auth(self.api_client.access_token())
            .query(&[("outletIds", outlet_id)])
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let licenses_response: LicenseResponse = response.json().await?;
                Ok(licenses_response.result.licenses)
            }
            _ => {
                let err: ErrorResponse = response.json().await?;
                let msg = format!("Error getting licenses\n{err:#?}");
                Err(msg.into())
            }
        }
    }
    // #[instrument]
    // pub async fn modify_license(&self) -> Result<()> {
    //     debug!("Modifying license");
    //     todo!()
    // }
    // #[instrument]
    // pub async fn delete_license(&self) -> Result<()> {
    //     debug!("Deleting license");
    //     todo!()
    // }
}
impl MarketClient {
    /// Точки продаж    
    /// # Example
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     // let outlets = client.outlets().get_all_outlets().await?;
    ///     // do something...
    ///     Ok(())
    /// }
    /// ```
    pub fn outlets(&self) -> Outlets {
        Outlets { api_client: self }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    #[tokio::test]
    async fn test_get_all_outlets() -> Result<()> {
        let client = MarketClient::init().await?;
        let outlets = client.outlets().get_all_outlets().await?;
        assert!(!outlets.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_outlet() -> Result<()> {
        let client = MarketClient::init().await?;
        let outlet = client.outlets().get_outlet(357750157).await?;
        assert_eq!(outlet.storage_period, 10);
        Ok(())
    }
    #[tokio::test]
    async fn test_create_update_delete_outlet() -> Result<()> {
        let client = MarketClient::init().await?;
        let address = crate::Address::builder()
            .region_id(13)
            .street("улица Ленина")
            .number("69")
            .block("5")
            .additional("Вход со двора")
            .build();
        let schedule_item_1 = crate::WorkingScheduleItem::builder()
            .start_day(crate::DayOfWeekType::Monday)
            .end_day(crate::DayOfWeekType::Friday)
            .start_time("09:00")
            .end_time("21:00")
            .build();
        let schedule_item_2 = crate::WorkingScheduleItem::builder()
            .start_day(crate::DayOfWeekType::Saturday)
            .end_day(crate::DayOfWeekType::Sunday)
            .start_time("10:00")
            .end_time("18:00")
            .build();
        let delivery_rule = crate::DeliveryRule::builder()
            .cost(0)
            .min_delivery_days(5)
            .max_delivery_days(7)
            .order_before(15)
            .build()?;
        let outlet_to_create = crate::Outlet::builder()
            .name("Test Outlet")
            .outlet_type(crate::OutletType::Retail)
            .coords("20.45, 54.71")
            .is_main(false)
            .shop_outlet_code("42")
            .visibility(crate::OutletVisibilityType::Hidden)
            .address(address)
            .phone("+7 (999) 696-69-69")
            .phone("+7 (888) 999-66-99")
            .phones(vec![
                "+7 (678) 321-65-49".to_string(),
                "+7 (987) 654-32-11".to_string(),
            ])
            .work_in_holiday(true)
            .schedule_item(schedule_item_1)
            .schedule_item(schedule_item_2)
            .delivery_rule(delivery_rule)
            .email("most@wanted.man")
            .storage_period(3)
            .build();
        let created = client.outlets().create_outlet(outlet_to_create).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let created_outlet = client.outlets().get_outlet(created).await?;
        let mut outlet_to_update = created_outlet;
        outlet_to_update.name = "Another name".to_string();
        client
            .outlets()
            .update_outlet(created, outlet_to_update)
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        client.outlets().delete_outlet(created).await?;
        Ok(())
    }
    #[tokio::test]
    async fn test_get_all_licenses() -> Result<()> {
        let client = MarketClient::init().await?;
        let outlets = client.outlets().get_all_outlets().await?;
        let id = outlets.first().unwrap().id;
        let licenses = client.outlets().get_all_licenses(id).await?;
        assert!(licenses.is_empty());
        Ok(())
    }
}
