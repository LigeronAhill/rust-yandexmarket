use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::{
    offer_cards::{NoOfferId, OfferErrorDTO},
    offer_mappings::{CurrencyId, GetPriceWithDiscountDTO},
    ApiResponseStatusType, ScrollingPagerDTO,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OffersRequest {
    pub offer_ids: Option<Vec<String>>,
    pub statuses: Option<Vec<OfferCampaignStatusType>>,
    pub category_ids: Option<Vec<i64>>,
    pub vendor_names: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferCampaignStatusType {
    Published,
    Checking,
    DisableByPartner,
    DisableByMarket,
    DisableAutomatically,
    CreatingCard,
    NoCard,
    NoStock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OffersResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<GetCampaignOffersResultDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCampaignOffersResultDTO {
    pub paging: Option<ScrollingPagerDTO>,
    pub offers: Option<Vec<GetCampaignOfferDTO>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCampaignOfferDTO {
    pub offer_id: String,
    pub quantum: Option<QuantumDTO>,
    pub available: Option<bool>,
    pub basic_price: Option<GetPriceWithDiscountDTO>,
    pub campaign_price: Option<GetPriceWithVatDTO>,
    pub status: Option<OfferCampaignStatusType>,
    pub errors: Option<Vec<OfferErrorDTO>>,
    pub warnings: Option<Vec<OfferErrorDTO>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuantumDTO {
    pub min_quantity: Option<i32>,
    pub step_quantity: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPriceWithVatDTO {
    pub value: Option<f64>,
    pub discount_base: Option<f64>,
    pub currency_id: Option<CurrencyId>,
    pub vat: Option<i32>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCampaignOffersRequest {
    pub offers: Vec<UpdateCampaignOfferDTO>,
}
/// Изменяет параметры размещения товаров в конкретном магазине: доступность товара, условия доставки и самовывоза, применяемую ставку НДС.
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCampaignOfferDTO {
    pub offer_id: String,
    pub quantum: Option<QuantumDTO>,
    pub available: Option<bool>,
    pub vat: Option<i32>,
}
impl UpdateCampaignOfferDTO {
    pub fn builder() -> UpdateCampaignOfferDTOBuilder<NoOfferId> {
        UpdateCampaignOfferDTOBuilder::default()
    }
}
#[derive(Default)]
pub struct UpdateCampaignOfferDTOBuilder<O> {
    offer_id: O,
    quantum: Option<QuantumDTO>,
    available: Option<bool>,
    vat: Option<i32>,
}
pub struct WithOfferId(String);
impl<O> UpdateCampaignOfferDTOBuilder<O> {
    pub fn offer_id(
        self,
        offer_id: impl Into<String>,
    ) -> UpdateCampaignOfferDTOBuilder<WithOfferId> {
        UpdateCampaignOfferDTOBuilder {
            offer_id: WithOfferId(offer_id.into()),
            quantum: self.quantum,
            available: self.available,
            vat: self.vat,
        }
    }
    pub fn min_quantity(mut self, min_quantity: i32) -> Self {
        let _ = self
            .quantum
            .get_or_insert(QuantumDTO::default())
            .min_quantity
            .insert(min_quantity);
        self
    }
    pub fn step_quantity(mut self, step_quantity: i32) -> Self {
        let _ = self
            .quantum
            .get_or_insert(QuantumDTO::default())
            .step_quantity
            .insert(step_quantity);
        self
    }
    pub fn available(mut self, available: bool) -> Self {
        let _ = self.available.insert(available);
        self
    }
    pub fn vat(mut self, vat: i32) -> Self {
        let _ = self.vat.insert(vat);
        self
    }
}
impl UpdateCampaignOfferDTOBuilder<WithOfferId> {
    pub fn build(self) -> UpdateCampaignOfferDTO {
        UpdateCampaignOfferDTO {
            offer_id: self.offer_id.0,
            quantum: self.quantum,
            available: self.available,
            vat: self.vat,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStockRequest {
    pub skus: Vec<StockDTO>,
}
/// Передает данные об остатках товаров на витрине.
/// Обязательно указывайте SKU в точности так, как он указан в каталоге. Например, 557722 и 0557722 — это два разных SKU.
///
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
///         .build()];
///     client.sales_managment().stock_update(stock).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockDTO {
    pub sku: String,
    pub warehouse_id: i64,
    pub items: Vec<StockItemDTO>,
}
impl StockDTO {
    pub fn builder() -> StockDTOBuilder<NoSku, NoWarehouseId, NoItems> {
        StockDTOBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockItemDTO {
    pub count: i64,
    #[serde(rename = "type")]
    pub stock_type: StockType,
    pub updated_at: DateTime<Local>,
}
impl StockItemDTO {
    pub fn new(count: i64) -> Self {
        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = chrono::DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        Self {
            count,
            stock_type: StockType::Fit,
            updated_at: dt_new,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StockType {
    Fit,
    Actual,
}

#[derive(Default)]
pub struct NoSku;
#[derive(Default)]
pub struct NoWarehouseId;
#[derive(Default)]
pub struct NoItems;
pub struct WithSku(String);
pub struct WithWarehouseId(i64);
pub struct WithItems(Vec<StockItemDTO>);
#[derive(Default)]
pub struct StockDTOBuilder<S, I, V> {
    sku: S,
    warehouse_id: I,
    items: V,
}
impl<S, I, V> StockDTOBuilder<S, I, V> {
    /// Ваш SKU товара.
    pub fn sku(self, sku: impl Into<String>) -> StockDTOBuilder<WithSku, I, V> {
        StockDTOBuilder {
            sku: WithSku(sku.into()),
            warehouse_id: self.warehouse_id,
            items: self.items,
        }
    }
    /// Идентификатор склада.
    /// Узнать идентификатор склада вы можете в личном кабинете в разделе Логистика → Склады. Он указан в поле ID склада.
    ///
    /// Если вы работаете с общими остатками, вы можете посмотреть идентификатор склада в личном кабинете в разделе Настройки → Настройки API в блоке Обновление данных об остатках товаров или с помощью запроса GET businesses/{businessId}/warehouses.
    pub fn warehouse_id(self, warehouse_id: i64) -> StockDTOBuilder<S, WithWarehouseId, V> {
        StockDTOBuilder {
            sku: self.sku,
            warehouse_id: WithWarehouseId(warehouse_id),
            items: self.items,
        }
    }
}
impl<S, I> StockDTOBuilder<S, I, NoItems> {
    /// Количество доступного товара.
    pub fn count(self, count: i64) -> StockDTOBuilder<S, I, WithItems> {
        let items = vec![StockItemDTO::new(count)];
        StockDTOBuilder {
            sku: self.sku,
            warehouse_id: self.warehouse_id,
            items: WithItems(items),
        }
    }
}
impl<S, I> StockDTOBuilder<S, I, WithItems> {
    /// Количество доступного товара.
    pub fn count(mut self, count: i64) -> Self {
        let x = self
            .items
            .0
            .first()
            .get_or_insert(&StockItemDTO::new(0))
            .count;
        let c = x + count;
        self.items.0 = vec![StockItemDTO::new(c)];
        self
    }
}
impl StockDTOBuilder<WithSku, WithWarehouseId, WithItems> {
    pub fn build(self) -> StockDTO {
        StockDTO {
            sku: self.sku.0,
            warehouse_id: self.warehouse_id.0,
            items: self.items.0,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockRequest {
    pub with_turnover: Option<bool>,
    pub archived: Option<bool>,
    pub offer_ids: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<GetWarehouseStocksDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWarehouseStocksDTO {
    pub paging: Option<ScrollingPagerDTO>,
    pub warehouses: Option<Vec<WarehouseOffersDTO>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseOffersDTO {
    pub warehouse_id: i64,
    pub offers: Vec<WarehouseOfferDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseOfferDTO {
    pub offer_id: String,
    pub turnover_summary: Option<TurnoverDTO>,
    pub stocks: Option<Vec<WarehouseStockDTO>>,
    pub updated_at: Option<DateTime<Local>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnoverDTO {
    pub turnover: TurnoverType,
    pub turnover_days: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseStockDTO {
    #[serde(rename = "type")]
    pub stock_type: WarehouseStockType,
    pub count: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TurnoverType {
    Low,
    AlmostLow,
    High,
    VeryHigh,
    NoSales,
    FreeStore,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WarehouseStockType {
    Available,
    Defect,
    Expired,
    Fit,
    Freeze,
    Quarantine,
    Utilization,
}
