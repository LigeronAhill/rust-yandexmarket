// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust

use anyhow::Result;
use reqwest::Url;
use secrecy::{ExposeSecret, Secret};
use tracing::instrument;

use crate::models::{
    CategoryDto, GetCategoriesMaxSaleQuantumRequest,
    GetCategoriesMaxSaleQuantumResponse, GetCategoriesResponse,
    GetCategoryContentParametersResponse,
};

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
