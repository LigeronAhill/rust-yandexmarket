use tracing::debug;

use crate::{
    models::{
        geobases::{RegionDTO, RegionsChildrenResponse, RegionsResponse},
        ErrorResponse,
    },
    MarketClient, Result,
};
/// Геобаза
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
///     let name = "Тамбов";
///     let regions = client.geobases().search_region(name).await?;
///     if let Some(first) = regions.first() {
///         let region = client.geobases().get_region(first.id).await?;
///         if let Some(first_region) = region.first() {
///             println!("Got region with type {}", first_region.region_type);
///         }
///         let children = client.geobases().get_children_regions(first.id).await?;
///         println!(
///             "region {} has {} children regions",
///             first.name,
///             children.len()
///         );
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Geobases<'a> {
    api_client: &'a MarketClient,
}
impl<'a> Geobases<'a> {
    /// Возвращает информацию о регионе, удовлетворяющем заданным в запросе условиям поиска.
    ///
    /// Если найдено несколько регионов, удовлетворяющих условиям поиска, возвращается информация по каждому найденному региону (но не более десяти регионов) для возможности определения нужного региона по родительским регионам.
    ///
    /// Для методов GET regions, GET regions/{regionId} и GET regions/{regionId}/children действует групповое ресурсное ограничение. Ограничение вводится на суммарное количество регионов, информация о которых запрошена при помощи этих методов (не более 100 000 регионов).
    ///
    /// Объем запросов к ресурсу, который возможно выполнить в течение суток, зависит от суммарного количества регионов.
    ///
    /// ⚙️ Лимит: 50 000 запросов в час
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
    ///     let name = "Тамбов";
    ///     let regions = client.geobases().search_region(name).await?;
    ///     if let Some(first) = regions.first() {
    ///         let region = client.geobases().get_region(first.id).await?;
    ///         if let Some(first_region) = region.first() {
    ///             println!("Got region with type {}", first_region.region_type);
    ///         }
    ///         let children = client.geobases().get_children_regions(first.id).await?;
    ///         println!(
    ///             "region {} has {} children regions",
    ///             first.name,
    ///             children.len()
    ///         );
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn search_region(&self, name: impl Into<String>) -> Result<Vec<RegionDTO>> {
        let name: String = name.into();
        debug!("Searching region {name}");
        let mut next_page_token = None;
        let mut result = Vec::new();
        loop {
            let uri = match next_page_token {
                Some(s) => {
                    format!("{}regions?name={name}&page_token={s}", crate::BASE_URL,)
                }
                None => {
                    format!("{}regions?name={name}", crate::BASE_URL)
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
                    let regions_response: RegionsResponse = response.json().await?;
                    result.extend(regions_response.regions);
                    match regions_response.paging {
                        Some(p) => match p.next_page_token {
                            Some(t) => next_page_token = Some(t),
                            None => break,
                        },
                        None => break,
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while searching region {name} --> \n{error:#?}",);
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
    /// Возвращает информацию о регионе.
    ///
    /// Для методов GET regions, GET regions/{regionId} и GET regions/{regionId}/children действует групповое ресурсное ограничение. Ограничение вводится на суммарное количество регионов, информация о которых запрошена при помощи этих методов (не более 100 000 регионов).
    ///
    /// Объем запросов к ресурсу, который возможно выполнить в течение суток, зависит от суммарного количества регионов.
    ///
    /// ⚙️ Лимит: 50 000 запросов в час
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
    ///     let name = "Тамбов";
    ///     let regions = client.geobases().search_region(name).await?;
    ///     if let Some(first) = regions.first() {
    ///         let region = client.geobases().get_region(first.id).await?;
    ///         if let Some(first_region) = region.first() {
    ///             println!("Got region with type {}", first_region.region_type);
    ///         }
    ///         let children = client.geobases().get_children_regions(first.id).await?;
    ///         println!(
    ///             "region {} has {} children regions",
    ///             first.name,
    ///             children.len()
    ///         );
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[tracing::instrument]
    pub async fn get_region(&self, region_id: i64) -> Result<Vec<RegionDTO>> {
        debug!("Searching region {region_id}");
        let mut next_page_token = None;
        let mut result = Vec::new();
        loop {
            let uri = match next_page_token {
                Some(s) => {
                    format!("{}regions/{region_id}&page_token={s}", crate::BASE_URL,)
                }
                None => {
                    format!("{}regions/{region_id}", crate::BASE_URL)
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
                    let regions_response: RegionsResponse = response.json().await?;
                    result.extend(regions_response.regions);
                    match regions_response.paging {
                        Some(p) => match p.next_page_token {
                            Some(t) => next_page_token = Some(t),
                            None => break,
                        },
                        None => break,
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while getting region {region_id} --> \n{error:#?}",);
                    return Err(msg.into());
                }
            }
        }
        Ok(result)
    }
    /// Возвращает информацию о регионах, являющихся дочерними по отношению к региону, идентификатор которого указан в запросе.
    ///
    /// Для методов GET regions, GET regions/{regionId} и GET regions/{regionId}/children действует групповое ресурсное ограничение. Ограничение вводится на суммарное количество регионов, информация о которых запрошена при помощи этих методов (не более 100 000 регионов).
    ///
    /// Объем запросов к ресурсу, который возможно выполнить в течение суток, зависит от суммарного количества регионов.
    ///
    /// ⚙️ Лимит: 50 000 запросов в час
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
    ///     let name = "Тамбов";
    ///     let regions = client.geobases().search_region(name).await?;
    ///     if let Some(first) = regions.first() {
    ///         let region = client.geobases().get_region(first.id).await?;
    ///         if let Some(first_region) = region.first() {
    ///             println!("Got region with type {}", first_region.region_type);
    ///         }
    ///         let children = client.geobases().get_children_regions(first.id).await?;
    ///         println!(
    ///             "region {} has {} children regions",
    ///             first.name,
    ///             children.len()
    ///         );
    ///     }
    ///     Ok(())
    /// }
    /// ```    
    pub async fn get_children_regions(&self, region_id: i64) -> Result<Vec<RegionDTO>> {
        let uri = format!("{}regions/{region_id}/children", crate::BASE_URL);
        let mut page = 1;
        let mut result = Vec::new();
        loop {
            let response = self
                .api_client
                .client()
                .get(&uri)
                .query(&[("page", page)])
                .bearer_auth(self.api_client.access_token())
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let regions_response: RegionsChildrenResponse = response.json().await?;
                    match regions_response.regions.children {
                        Some(c) => result.extend(c),
                        None => todo!(),
                    }
                    if regions_response.pager.pages_count.is_some_and(|p| p > page) {
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
}
impl MarketClient {
    /// Геобаза
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
    ///     let name = "Тамбов";
    ///     let regions = client.geobases().search_region(name).await?;
    ///     if let Some(first) = regions.first() {
    ///         let region = client.geobases().get_region(first.id).await?;
    ///         if let Some(first_region) = region.first() {
    ///             println!("Got region with type {}", first_region.region_type);
    ///         }
    ///         let children = client.geobases().get_children_regions(first.id).await?;
    ///         println!(
    ///             "region {} has {} children regions",
    ///             first.name,
    ///             children.len()
    ///         );
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn geobases(&self) -> Geobases {
        Geobases { api_client: self }
    }
}
#[cfg(test)]
mod tests {
    use crate::{MarketClient, Result};
    #[tokio::test]
    async fn test_get_all_campaigns() -> Result<()> {
        let client = MarketClient::init().await?;
        let campaigns = client.campaigns().get_all_campaigns().await?;
        assert!(!campaigns.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_campaign() -> Result<()> {
        let client = MarketClient::init().await?;
        let campaign = client
            .campaigns()
            .get_campaign(client.campaign_id())
            .await?;
        assert_eq!(campaign.id, client.campaign_id());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_logins() -> Result<()> {
        let client = MarketClient::init().await?;
        let logins = client.campaigns().get_logins(client.campaign_id()).await?;
        assert!(!logins.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_logins_campaigns() -> Result<()> {
        let client = MarketClient::init().await?;
        let login = client
            .campaigns()
            .get_logins(client.campaign_id())
            .await?
            .first()
            .unwrap()
            .to_string();
        let campaigns = client.campaigns().get_logins_campaigns(login).await?;
        assert!(!campaigns.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_campaign_settings() -> Result<()> {
        let client = MarketClient::init().await?;
        let settings = client
            .campaigns()
            .get_settings(client.campaign_id())
            .await?;
        assert!(settings.show_in_premium);
        Ok(())
    }
}
