// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust

use crate::models::{
    CategoryDto, GetCampaignsResponse, GetCategoriesMaxSaleQuantumRequest,
    GetCategoriesMaxSaleQuantumResponse, GetCategoriesResponse,
    GetCategoryContentParametersResponse, UpdateOfferMappingDto, UpdateOfferMappingsRequest,
    UpdateOfferMappingsResponse,
};
use anyhow::Result;
use reqwest::Url;
use secrecy::{ExposeSecret, Secret};
use std::fmt::{Debug, Display};
use tracing::instrument;

pub mod models;

/// Клиент для доступа к Yandex Market API.
///
/// # Пример
///
/// ```rust
///use anyhow::Result;
///use rust_yandexmarket::MarketClient;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let subscriber = tracing_subscriber::fmt()
///         .with_max_level(tracing::Level::DEBUG)
///         .finish();
///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
///     let client = MarketClient::new(token)?;
///     // do something with the client
///     Ok(())
/// }
///```
#[derive(Debug)]
pub struct MarketClient {
    token: Secret<String>,
    base_url: Url,
    client: reqwest::Client,
}
impl MarketClient {
    /// Создает новый экземпляр `MarketClient`.
    ///
    /// # Аргументы
    ///
    /// * `token` - API токен для авторизации..
    ///
    /// # Возвращаемое значение
    ///
    /// `Result` который может быть `Ok(Self)` или `Err(Error)`.
    ///
    ///
    /// # Пример
    ///
    /// ```rust
    ///use anyhow::Result;
    ///use rust_yandexmarket::MarketClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     // do something with the client
    ///     Ok(())
    /// }
    ///```
    #[instrument(skip_all)]
    pub fn new<T: ToString>(token: T) -> Result<Self> {
        let token = Secret::new(token.to_string());
        let base_url = Url::parse("https://api.partner.market.yandex.ru")?;
        let client = reqwest::Client::builder().gzip(true).build()?;
        Ok(Self {
            token,
            base_url,
            client,
        })
    }
    #[instrument(skip(self))]
    fn token(&self) -> String {
        self.token.expose_secret().to_string()
    }
    /// Асинхронно получает дерево категорий.
    ///
    /// # Возвращаемое значение
    ///
    /// `Result` который может быть `Ok(GetCategoriesResponse)` - структура, представляющая ответ на запрос категорий или `Err(Error)`
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     let categories_tree = client.get_categories_tree().await?;
    ///     info!("Categories tree: {:#?}", categories_tree);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_categories_tree(&self) -> Result<GetCategoriesResponse> {
        let uri = self.base_url.join("categories/tree")?;
        let result: GetCategoriesResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token())
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    /// Возвращает список характеристик с допустимыми значениями для заданной категории.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     info!("Client initialized successfully\n{client:#?}");
    ///     let category_id = 91636;
    ///     let category_content_parameters = client.get_category_content_parameters(category_id).await?;
    ///     info!("Category content parameters: {:#?}", category_content_parameters);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_category_content_parameters(
        &self,
        category_id: i64,
    ) -> Result<GetCategoryContentParametersResponse> {
        let endpoint = format!("category/{category_id}/parameters");
        let uri = self.base_url.join(&endpoint)?;
        let result: GetCategoryContentParametersResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token())
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    /// Возвращает лимит на установку кванта и минимального количества товаров в заказе, которые вы можете задать для товаров указанных категорий.
    ///
    /// Если вы передадите значение кванта или минимального количества товаров выше установленного Маркетом ограничения, товар будет скрыт с витрины.
    ///
    /// # Пример
    ///
    /// ```rust
    ///
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     info!("Client initialized successfully\n{client:#?}");
    ///     let category_id = 91636;
    ///     let category_max_sale_quantum = client.get_categories_max_sale_quantum(vec![category_id]).await?;
    ///     info!("Category max sale quantum:\n{category_max_sale_quantum:#?}");
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_categories_max_sale_quantum(
        &self,
        categories: Vec<i64>,
    ) -> Result<GetCategoriesMaxSaleQuantumResponse> {
        let uri = self.base_url.join("categories/max-sale-quantum")?;
        let body = GetCategoriesMaxSaleQuantumRequest::new(categories);
        let response: GetCategoriesMaxSaleQuantumResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token())
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    /// Возвращает список магазинов, к которым имеет доступ пользователь — владелец авторизационного токена, использованного в запросе. Для агентских пользователей список состоит из подагентских магазинов.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     info!("Client initialized successfully\n{client:#?}");
    ///     let campaigns = client.get_campaigns(None, None).await?;
    ///     info!("Campaigns: {:#?}", campaigns);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_campaigns(
        &self,
        page: Option<u16>,
        page_size: Option<u16>,
    ) -> Result<GetCampaignsResponse> {
        let uri = self.base_url.join("campaigns")?;
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);
        let response: GetCampaignsResponse = self
            .client
            .get(uri)
            .query(&[("page", page), ("page_size", page_size)])
            .bearer_auth(&self.token())
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    /// Добавляет товары в каталог, передает их категории на Маркете и характеристики, необходимые для этих категории. Также редактирует информацию об уже имеющихся товарах.
    ///
    /// Список категорий Маркета можно получить с помощью запроса get_categories_tree, а характеристики товаров по категориям с помощью get_category_content_parameters.
    ///
    /// Чтобы добавить новый товар, передайте его с новым идентификатором, который раньше никогда не использовался в каталоге. Старайтесь сразу передать как можно больше информации — она потребуется Маркету для подбора подходящей карточки или создания новой. Если известно, какой карточке на Маркете соответствует товар, можно сразу указать идентификатор этой карточки (SKU на Маркете) в поле marketSKU.
    ///
    /// Для новых товаров обязательно укажите параметры: offerId, name, marketCategoryId или category, pictures, vendor, description.
    ///
    /// Чтобы отредактировать информацию о товаре, передайте новые данные, указав в offerId соответствующий ваш SKU. Поля, в которых ничего не меняется, можно не передавать.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::models::{CategoryParameterDto, ParameterValueDto, UpdateOfferDto, UpdateOfferMappingDto};
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     info!("Client initialized successfully\n{client:#?}");
    ///     let business_id = 919862;
    ///     let category_id = 6119048;
    ///     let content_parameters = client
    ///         .get_category_content_parameters(category_id)
    ///         .await?
    ///         .result
    ///         .and_then(|r| r.parameters)
    ///         .unwrap_or_default();
    ///     let mut parameters = Vec::new();
    ///     let get_unit_id = |c: &CategoryParameterDto, unit: &str| {
    ///         c.clone().unit.and_then(|u| {
    ///             u
    ///                 .units
    ///                 .into_iter()
    ///                 .find(|d| d.full_name == unit)
    ///                 .map(|d| d.id)
    ///         })
    ///     };
    ///     for content_parameter in content_parameters {
    ///         if let Some(name) = content_parameter.name {
    ///             let parameter_id = content_parameter.id;
    ///             let mut unit_id = None;
    ///             let mut value_id = None;
    ///             let value = match name.as_str() {
    ///                 "Ширина" => {
    ///                     unit_id = get_unit_id(&content_parameter, "метр");
    ///                     "0.8"
    ///                 }
    ///                 "Форма" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "прямоугольная"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "прямоугольная"
    ///                 }
    ///                 "Цвет товара для карточки" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "75"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "75"
    ///                 }
    ///                 "Количество в наборе" => "2",
    ///                 "Длина" => "1.5",
    ///                 "Цвет товара для фильтра" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "серый"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "серый"
    ///                 }
    ///                 "Вес" => "6.72",
    ///                 "Толщина" => {
    ///                     unit_id = get_unit_id(&content_parameter, "миллиметр");
    ///                     "15.5"
    ///                 }
    ///                 "Материал основы" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "джут"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "джут"
    ///                 }
    ///                 "Материал верха" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "полиамид"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "полиамид"
    ///                 }
    ///                 "Тип" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "ковер"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "ковер"
    ///                 }
    ///                 "Тип рисунка" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "однотонный"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "однотонный"
    ///                 }
    ///                 "Способ производства" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "машинный"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "машинный"
    ///                 }
    ///                 "Противоскользящая основа" => "false",
    ///                 "Безворсовый" => "false",
    ///                 "Вес ворса на квадратный метр" => {
    ///                     unit_id = get_unit_id(&content_parameter, "г/м²");
    ///                     "2100"
    ///                 }
    ///                 "Высота ворса" => {
    ///                     unit_id = get_unit_id(&content_parameter, "миллиметр");
    ///                     "13"
    ///                 }
    ///                 "Вес на квадратный метр" => {
    ///                     unit_id = get_unit_id(&content_parameter, "г/м²");
    ///                     "2800"
    ///                 }
    ///                 "Страна производства" => {
    ///                     value_id = content_parameter
    ///                         .values
    ///                         .and_then(|v| v.into_iter().find(|p| p.value == "Бельгия"))
    ///                         .map(|p| p.id.to_owned());
    ///                     "Бельгия"
    ///                 }
    ///                 "Набор" => "true",
    ///                 _ => continue,
    ///             };
    ///             let pvd = ParameterValueDto::build()
    ///                 .parameter_id(parameter_id)
    ///                 .unit_id(unit_id)
    ///                 .value_id(value_id)
    ///                 .value(value)
    ///                 .build()?;
    ///             parameters.push(pvd);
    ///         }
    ///     }
    ///     let pictures = vec![
    ///         "https://safira.club/wp-content/uploads/mekota_75_large.jpeg".to_string(),
    ///         "https://safira.club/wp-content/uploads/mekota_75_office_large.jpeg".to_string(),
    ///     ];
    ///     let offer = UpdateOfferDto::builder()
    ///         .offer_id("AW Carolus 75 0.8x1.5 - 2 pcs")
    ///         .name("Ковер AW Carolus 75 0.8x1.5 м комплект 2 штуки")
    ///         .market_category_id(category_id)
    ///         .category("Ковры")
    ///         .pictures(pictures)
    ///         .vendor("AW")
    ///         .description("Ковёр AW Carolus – это качественный и эстетичный элемент декора из Бельгии. Это однотонный ковёр из 100% полиэстера с окантованными неширокой тесьмой краями.")
    ///         .manufacturer_countries(vec!["Бельгия".to_string()])
    ///         .weight_dimensions(80.0, 40.0, 40.0, 6.72)
    ///         .vendor_code("AW Carolus 75")
    ///         .parameter_values(parameters)
    ///         .basic_price(21990.0, Some(27490.0))
    ///         .purchase_price(15340.0)
    ///         .additional_expenses(2900.0)
    ///         .cofinance_price(20190.0)
    ///         .build()?;
    ///     let update_offers_mapping_dto = vec![UpdateOfferMappingDto::new(offer)];
    ///     let update_result = client
    ///         .update_offer_mappings(business_id, update_offers_mapping_dto)
    ///         .await?;
    ///     info!("Update result:\n{:#?}", update_result);
    ///     Ok(())
    /// }
    ///```
    #[instrument(skip(self, offers))]
    pub async fn update_offer_mappings<T: Debug + Display>(
        &self,
        business_id: T,
        offers: Vec<UpdateOfferMappingDto>,
    ) -> Result<UpdateOfferMappingsResponse> {
        let endpoint = format!("businesses/{business_id}/offer-mappings/update");
        let uri = self.base_url.join(&endpoint)?;
        let body = UpdateOfferMappingsRequest::new(offers);
        let response: UpdateOfferMappingsResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token())
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
pub fn search_in_categories_by_name(
    categories: &[CategoryDto],
    search_string: &str,
) -> Option<CategoryDto> {
    for category in categories {
        if category
            .name
            .to_lowercase()
            .contains(&search_string.to_lowercase())
        {
            return Some(category.clone());
        } else {
            if let Some(children) = category.children.clone() {
                if let Some(category) = search_in_categories_by_name(&children, search_string) {
                    return Some(category);
                }
            }
        }
    }
    None
}
