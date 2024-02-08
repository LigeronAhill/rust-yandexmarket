use chrono::NaiveDateTime;
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
