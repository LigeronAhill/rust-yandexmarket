// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust

use anyhow::Result;
use reqwest::Url;
use secrecy::{ExposeSecret, Secret};
use tracing::{error, instrument};

use crate::models::{ApiErrorResponse, ApiResponseStatusType, CalculateTariffsOfferDto, CalculateTariffsParametersDto, CalculateTariffsRequest, CalculateTariffsResponse, CampaignDto, ConfirmPricesRequest, DeleteOffersRequest, DeleteOffersResponse, GetCampaignOfferDto, GetCampaignOffersRequest, GetCampaignOffersResponse, GetCampaignsResponse, GetCategoriesMaxSaleQuantumRequest, GetCategoriesMaxSaleQuantumResponse, GetCategoriesResponse, GetCategoryContentParametersResponse, GetOfferMappingDto, GetOfferMappingsRequest, GetOfferMappingsResponse, GetOfferRecommendationsRequest, GetOfferRecommendationsResponse, GetQuarantineOffersRequest, GetQuarantineOffersResponse, OfferRecommendationDto, PaymentFrequencyType, QuarantineOfferDto, SellingProgramType, UpdateBusinessOfferPriceDto, UpdateBusinessPricesRequest, UpdateCampaignOfferDto, UpdateCampaignOffersRequest, UpdateOfferMappingDto, UpdateOfferMappingsRequest, UpdateOfferMappingsResponse};

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
///
/// ## Как добавить новые товары в каталог
///
/// 1. Получите список категорий Маркета, выполнив запрос [`get_categories_tree`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/categories/getCategoriesTree).
/// 2. Для каждой категории запросите список необходимых характеристик с помощью [`get_category_content_parameters`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/content/getCategoryContentParameters).
/// 3. Передайте информацию о товарах (названия, описания, фотографии и так далее), цены, категории на Маркете и характеристики с помощью запроса [`update_offer_mappings`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/updateOfferMappings).
/// 4. Чтобы узнать стоимость услуг Маркета для конкретных товаров, передайте их параметры в запросе [`tariffs_calculate`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/tariffs/calculateTariffs).
/// 5. Получите у Маркета список моделей, по которым можно продавать каждый из добавленных товаров с помощью запроса [`offer_mappings`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/getOfferMappings). [Что такое модель работы и какие модели есть](https://yandex.ru/support/marketplace/introduction/models.html).
/// 6. Задайте условия размещения товаров с помощью запроса [`offers_update`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/assortment/updateCampaignOffers). Условия размещения — это минимальный объем заказа, квант продаж и ставка НДС. Если вы работаете по модели DBS, этим же запросом задаются параметры доставки.
/// 7. Убедитесь, что товары появились на витрине, с помощью запроса [`get_campaign_offers`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/assortment/getCampaignOffers).
/// Подробные пояснения к статусам товаров вы найдете в [Справке Маркета для продавцов](https://yandex.ru/support/marketplace/assortment/add/statuses.html).
///
/// ## Как изменить цены на товары
///
/// 1. Чтобы узнать стоимость услуг Маркета для конкретных товаров, передайте их параметры в запросе [`tariffs_calculate`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/tariffs/calculateTariffs).
/// 2. Передайте новые цены для всех магазинов с помощью запроса [`offer_prices_updates`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/updateBusinessPrices).
/// 3. Убедитесь, что ни один из товаров не попал в карантин с помощью запроса ['price_quarantine'](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/getBusinessQuarantineOffers).
/// 4. Если карантин не пустой, проверьте цены на товары. Ошибочно установленные цены для всех магазинов можно исправить запросом [`offer_prices_updates`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/updateBusinessPrices).
/// 5. После того как в карантине останутся только правильные цены, подтвердите их запросом ['price_quarantine_confirm'](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/confirmBusinessPrices). Если ложные срабатывания карантина случаются часто, подумайте о том, чтобы изменить его порог по [инструкции](https://yandex.ru/support/marketplace/assortment/operations/prices.html#quarantine).
#[derive(Debug)]
pub struct MarketClient {
    token: Secret<String>,
    base_url: Url,
    client: reqwest::Client,
}
impl MarketClient {
    /// Создает новый экземпляр `MarketClient`.
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
    ///     let campaigns = client.get_campaigns().await?;
    ///     info!("Campaigns: {:#?}", campaigns);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_campaigns(&self) -> Result<Vec<CampaignDto>> {
        let uri = self.base_url.join("campaigns")?;
        let mut page = 1;
        let page_size = 10;
        let mut result = Vec::new();
        loop {
            let response: GetCampaignsResponse = self
                .client
                .get(uri.clone())
                .query(&[("page", page), ("page_size", page_size)])
                .bearer_auth(&self.token())
                .send()
                .await?
                .json()
                .await?;
            match response.campaigns {
                None => {
                    break;
                }
                Some(campaigns) => {
                    result.extend(campaigns);
                    match response.pager.and_then(|p| p.pages_count) {
                        None => {
                            break;
                        }
                        Some(count) => {
                            if page < count {
                                page += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Ok(result)
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
    pub async fn update_offer_mappings(
        &self,
        business_id: i64,
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
    /// Рассчитывает стоимость услуг Маркета для товаров с заданными параметрами. Порядок товаров в запросе и ответе сохраняется, чтобы определить, для какого товара рассчитана стоимость услуги.
    ///
    /// Обратите внимание: калькулятор осуществляет примерные расчеты. Финальная стоимость для каждого заказа зависит от предоставленных услуг.
    ///
    /// В запросе можно указать либо параметр `campaignId`, либо `sellingProgram`. Совместное использование параметров приведет к ошибке.
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    /// use rust_yandexmarket::models::{CalculateTariffsOfferDto, SellingProgramType};
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
    ///     let offers = vec![CalculateTariffsOfferDto::new(
    ///         6119048,
    ///         21990.0,
    ///         0.8,
    ///         0.4,
    ///         0.4,
    ///         6.72,
    ///         Some(1),
    ///     )];
    ///     let tariffs = client
    ///         .tariffs_calculate(None, Some(SellingProgramType::Dbs), None, offers)
    ///         .await?;
    ///     info!("Tariffs: {:#?}", tariffs);
    ///     Ok(())
    /// }
    ///```
    #[instrument(skip(self, offers))]
    pub async fn tariffs_calculate(
        &self,
        campaign_id: Option<i64>,
        selling_program: Option<SellingProgramType>,
        frequency: Option<PaymentFrequencyType>,
        offers: Vec<CalculateTariffsOfferDto>,
    ) -> Result<CalculateTariffsResponse> {
        let uri = self.base_url.join("tariffs/calculate")?;
        let parameters =
            CalculateTariffsParametersDto::new(campaign_id, selling_program, frequency);
        let body = CalculateTariffsRequest::new(parameters, offers);
        let response: CalculateTariffsResponse = self
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
    /// Возвращает список товаров в каталоге, их категории на Маркете и характеристики каждого товара.
    /// Можно использовать тремя способами:
    ///
    /// Задать список интересующих SKU;
    /// задать фильтр — в этом случае результаты возвращаются постранично;
    /// не передавать тело запроса, чтобы получить список всех товаров в каталоге.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    /// use rust_yandexmarket::models::GetOfferMappingsRequest;
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
    ///     let request = Some(GetOfferMappingsRequest::builder()
    ///         .vendor_names(vec!["Haima".to_string()])
    ///         .build()?);
    ///     let offer_mappings = client.offer_mappings(business_id, None).await?;
    ///     info!("Offer mappings: {:#?}", offer_mappings);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn offer_mappings(
        &self,
        business_id: i64,
        body: Option<GetOfferMappingsRequest>,
    ) -> Result<Vec<GetOfferMappingDto>> {
        let endpoint = format!("businesses/{business_id}/offer-mappings");
        let mut uri = self.base_url.join(&endpoint)?;
        uri.set_query(Some("limit=200"));
        let mut page_token = None;
        let mut result = Vec::new();
        loop {
            if let Some(next_page_token) = page_token.clone() {
                let query = format!("page_token={next_page_token}");
                uri.set_query(Some(query.as_str()))
            }
            let response: GetOfferMappingsResponse = if let Some(body) = body.clone() {
                self.client
                    .post(uri.clone())
                    .bearer_auth(&self.token())
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?
            } else {
                self.client
                    .post(uri.clone())
                    .bearer_auth(&self.token())
                    .send()
                    .await?
                    .json()
                    .await?
            };
            let offer_mappings = response
                .result
                .clone()
                .and_then(|r| r.offer_mappings)
                .unwrap_or_default();
            result.extend(offer_mappings);
            if let Some(next_page_token) = response
                .result
                .and_then(|r| r.paging.and_then(|p| p.next_page_token))
            {
                page_token = Some(next_page_token);
            } else {
                break;
            }
        }
        Ok(result)
    }
    /// Изменяет параметры размещения товаров в конкретном магазине: доступность товара, условия доставки и самовывоза, применяемую ставку НДС.
    ///
    /// # Пример
    ///
    /// ```rust
    ///
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    /// use rust_yandexmarket::models::UpdateCampaignOfferDto;
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
    ///     let campaign_id = 112289474;
    ///     let offer = UpdateCampaignOfferDto::builder()
    ///         .offer_id("AW Carolus 75 0.8x1.5 - 2 pcs")
    ///         .quantum(1,1)
    ///         .available(true)
    ///         .vat(6)
    ///         .build()?;
    ///     let result = client.offers_update(campaign_id, vec![offer]).await?;
    ///     info!("Offers update result:\n{result:#?}");
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, offers))]
    pub async fn offers_update(
        &self,
        campaign_id: i64,
        offers: Vec<UpdateCampaignOfferDto>,
    ) -> Result<ApiErrorResponse> {
        let endpoint = format!("campaigns/{}/offers/update", campaign_id);
        let uri = self.base_url.join(&endpoint)?;
        let body = UpdateCampaignOffersRequest::new(offers);
        let result: ApiErrorResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token())
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    /// Возвращает список товаров, размещенных в заданном магазине. Для каждого товара указываются параметры размещения.
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
    ///     let offers = client
    ///         .get_campaign_offers(112289474, None)
    ///         .await?;
    ///     info!("Offers: {:#?}", offers);
    ///     Ok(())
    /// }
    ///```
    #[instrument(skip(self))]
    pub async fn get_campaign_offers(
        &self,
        campaign_id: i64,
        get_campaign_offers_request: Option<GetCampaignOffersRequest>,
    ) -> Result<Vec<GetCampaignOfferDto>> {
        let endpoint = format!("campaigns/{}/offers", campaign_id);
        let mut uri = self.base_url.join(&endpoint)?;
        uri.set_query(Some("limit=200"));
        let mut page_token = None;
        let mut result = Vec::new();
        loop {
            if let Some(next_page_token) = page_token {
                let query = format!("page_token={}", next_page_token);
                uri.set_query(Some(query.as_str()));
            }
            let body = get_campaign_offers_request.clone().unwrap_or_default();
            let response: GetCampaignOffersResponse = self
                .client
                .post(uri.clone())
                .bearer_auth(&self.token())
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            let offers = response
                .result
                .clone()
                .and_then(|r| r.offers)
                .unwrap_or_default();
            result.extend(offers);
            match response
                .result
                .and_then(|r| r.paging.and_then(|s| s.next_page_token))
            {
                None => break,
                Some(next_page_token) => {
                    page_token = Some(next_page_token);
                }
            }
        }
        Ok(result)
    }
    /// Удаляет товары из каталога.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use tracing::info;
    /// use rust_yandexmarket::models::GetOfferMappingsRequest;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     let business_id = 919862;
    ///     let req = GetOfferMappingsRequest::builder()
    ///         .vendor_names(vec!["Haima".to_string()])
    ///         .build()?;
    ///     let offer_ids = client.offer_mappings(business_id, Some(req)).await?.into_iter().flat_map(|o| o.offer.map(|f| f.offer_id)).collect::<Vec<_>>();
    ///     let not_deleted_offers = client.delete_offers(business_id, offer_ids).await?;
    ///     info!("Not deleted offers: {not_deleted_offers:#?}");
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, offer_ids))]
    pub async fn delete_offers(
        &self,
        business_id: i64,
        offer_ids: Vec<String>,
    ) -> Result<Vec<String>> {
        let endpoint = format!("businesses/{}/offer-mappings/delete", business_id);
        let uri = self.base_url.join(&endpoint)?;
        let mut not_deleted_offers = Vec::new();
        for offer_ids_chunk in offer_ids.chunks(200) {
            let body = DeleteOffersRequest::new(offer_ids_chunk.to_vec());
            let result: DeleteOffersResponse = self
                .client
                .post(uri.clone())
                .bearer_auth(&self.token())
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            let not_deleted = result
                .result
                .clone()
                .and_then(|r| r.not_deleted_offer_ids)
                .unwrap_or_default();
            not_deleted_offers.extend(not_deleted);
            if let Some(status) = result.status {
                match status {
                    ApiResponseStatusType::Ok => {}
                    ApiResponseStatusType::Error => {
                        error!("Error deleting offers: {:#?}", result);
                        break;
                    }
                }
            }
        }
        Ok(not_deleted_offers)
    }
    /// Устанавливает базовые цены. Чтобы получить рекомендации Маркета, касающиеся цен, выполните запрос [`offers_recommendations`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/getOfferRecommendations)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_yandexmarket::MarketClient;
    /// use rust_yandexmarket::models::UpdateBusinessOfferPriceDto;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let subscriber = tracing_subscriber::fmt()
    ///         .with_max_level(tracing::Level::DEBUG)
    ///         .finish();
    ///     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    ///     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    ///     let client = MarketClient::new(token)?;
    ///     let business_id = 919862;
    ///     let offer_id = "AW Mambo 99 50x50";
    ///     let body = vec![UpdateBusinessOfferPriceDto::new(offer_id, 26925.0, Some(31690.0))];
    ///     client.offer_prices_updates(business_id, body).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, offers))]
    pub async fn offer_prices_updates(
        &self,
        business_id: i64,
        offers: Vec<UpdateBusinessOfferPriceDto>,
    ) -> Result<()> {
        let endpoint = format!("businesses/{business_id}/offer-prices/updates");
        let uri = self.base_url.join(&endpoint)?;
        for chunk in offers.chunks(500) {
            let body = UpdateBusinessPricesRequest::new(chunk.to_vec());
            let result: ApiErrorResponse = self
                .client
                .post(uri.clone())
                .bearer_auth(&self.token())
                .json(&body)
                .send()
                .await?
                .json()
                .await?;

            if result
                .status
                .is_some_and(|s| s == ApiResponseStatusType::Error)
            {
                error!("Error:\n{:?}", result.errors)
            }
        }
        Ok(())
    }
    /// Рекомендации Маркета, касающиеся цен
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
    ///     let recommendations = client.offers_recommendations(919862, None).await?;
    ///     info!("{recommendations:#?}");
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, offer_ids))]
    pub async fn offers_recommendations(
        &self,
        business_id: i64,
        offer_ids: Option<Vec<String>>,
    ) -> Result<Vec<OfferRecommendationDto>> {
        let endpoint = format!("businesses/{business_id}/offers/recommendations");
        let mut uri = self.base_url.join(&endpoint)?;
        uri.set_query(Some("limit=200"));
        let mut page_token = None;
        let mut result = Vec::new();
        let body = GetOfferRecommendationsRequest::new(offer_ids);
        loop {
            if let Some(next_page_token) = page_token {
                let query = format!("page_token={}", next_page_token);
                uri.set_query(Some(query.as_str()));
            }
            let response: GetOfferRecommendationsResponse = self
                .client
                .post(uri.clone())
                .bearer_auth(&self.token())
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            let offers = response
                .result
                .clone()
                .and_then(|r| r.offer_recommendations)
                .unwrap_or_default();
            result.extend(offers);
            match response
                .result
                .and_then(|r| r.paging.and_then(|s| s.next_page_token))
            {
                None => break,
                Some(next_page_token) => {
                    page_token = Some(next_page_token);
                }
            }
        }
        Ok(result)
    }
    /// Возвращает список товаров, которые находятся на карантине по основной цене. Основная цена задается в каталоге и действует во всех магазинах кабинета.
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
    ///     let business_id = 919862;
    ///     let quarantine = client.price_quarantine(business_id, None).await?;
    ///     info!("{quarantine:#?}");
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self))]
    pub async fn price_quarantine(
        &self,
        business_id: i64,
        req: Option<GetQuarantineOffersRequest>
    ) -> Result<Vec<QuarantineOfferDto>> {
        let endpoint = format!("businesses/{business_id}/price-quarantine");
        let mut uri = self.base_url.join(&endpoint)?;
        uri.set_query(Some("limit=200"));
        let mut page_token = None;
        let mut result = Vec::new();
        let body = req.unwrap_or(GetQuarantineOffersRequest::new());
        loop {
            if let Some(next_page_token) = page_token {
                let query = format!("page_token={}", next_page_token);
                uri.set_query(Some(query.as_str()));
            }
            let response: GetQuarantineOffersResponse = self
                .client
                .post(uri.clone())
                .bearer_auth(&self.token())
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            let offers = response
                .result
                .clone()
                .and_then(|r| r.offers)
                .unwrap_or_default();
            result.extend(offers);
            match response
                .result
                .and_then(|r| r.paging.and_then(|s| s.next_page_token))
            {
                None => break,
                Some(next_page_token) => {
                    page_token = Some(next_page_token);
                }
            }
        }
        Ok(result)
    }
    /// Подтверждает основную цену на товары, которые попали в карантин.
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
    ///     let business_id = 919862;
    ///     let quarantine = client.price_quarantine(business_id, None).await?;
    ///     info!("{quarantine:#?}");
    ///     let offer_ids = quarantine
    ///         .into_iter()
    ///         .flat_map(|q| q.offer_id)
    ///         .collect::<Vec<_>>();
    ///     client.price_quarantine_confirm(business_id, offer_ids).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, offer_ids))]
    pub async fn price_quarantine_confirm(
        &self,
        business_id: i64,
        offer_ids: Vec<String>,
    ) -> Result<()> {
        let endpoint = format!("businesses/{business_id}/price-quarantine/confirm");
        let uri = self.base_url.join(&endpoint)?;
        for chunk in offer_ids.chunks(200) {
            let body = ConfirmPricesRequest::new(chunk.to_vec());
            let response: ApiErrorResponse = self
                .client
                .post(uri.clone())
                .bearer_auth(self.token())
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            if response
                .status
                .is_some_and(|s| s == ApiResponseStatusType::Error)
            {
                error!("Error:\n{:?}", response.errors)
            }
        }
        Ok(())
    }
}
// pub fn search_in_categories_by_name(
//     categories: &[CategoryDto],
//     search_string: &str,
// ) -> Option<CategoryDto> {
//     for category in categories {
//         if category
//             .name
//             .to_lowercase()
//             .contains(&search_string.to_lowercase())
//         {
//             return Some(category.clone());
//         } else {
//             if let Some(children) = category.children.clone() {
//                 if let Some(category) = search_in_categories_by_name(&children, search_string) {
//                     return Some(category);
//                 }
//             }
//         }
//     }
//     None
// }
