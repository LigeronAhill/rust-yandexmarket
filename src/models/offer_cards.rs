use crate::models::offer_mappings::GetMappingDTO;
use crate::models::{ApiResponseStatusType, ForwardScrollingPagerDTO};
use crate::OfferCardStatusType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferCardResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<OfferCardContentStatusDTO>,
}
/// Фильтрация для запроса карточек товаров
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferCardRequest {
    pub offer_ids: Option<Vec<String>>,
    pub card_statuses: Option<Vec<OfferCardStatusType>>,
    pub category_ids: Option<Vec<i64>>,
}
impl OfferCardRequest {
    pub fn builder() -> OfferCardRequestBuilder {
        OfferCardRequestBuilder::default()
    }
}
#[derive(Default)]
pub struct OfferCardRequestBuilder {
    pub offer_ids: Option<Vec<String>>,
    pub card_statuses: Option<Vec<OfferCardStatusType>>,
    pub category_ids: Option<Vec<i64>>,
}
impl OfferCardRequestBuilder {
    /// Идентификаторы товаров, информация о которых нужна.
    pub fn offer_ids(&mut self, offer_ids: Vec<String>) -> &mut Self {
        self.offer_ids.get_or_insert(vec![]).extend(offer_ids);
        self
    }
    /// Идентификатор товара, информация о котором нужна.
    pub fn offer_id(&mut self, offer_id: impl Into<String>) -> &mut Self {
        self.offer_ids.get_or_insert(vec![]).push(offer_id.into());
        self
    }
    /// Фильтр по статусам карточек.
    pub fn card_statuses(&mut self, card_statuses: Vec<OfferCardStatusType>) -> &mut Self {
        self.card_statuses
            .get_or_insert(vec![])
            .extend(card_statuses);
        self
    }
    /// Фильтр по статусам карточек.
    pub fn card_status(&mut self, card_status: OfferCardStatusType) -> &mut Self {
        self.card_statuses.get_or_insert(vec![]).push(card_status);
        self
    }
    /// Фильтр по категориям на Маркете.
    pub fn category_ids(&mut self, category_ids: Vec<i64>) -> &mut Self {
        self.category_ids.get_or_insert(vec![]).extend(category_ids);
        self
    }
    /// Фильтр по категориям на Маркете.
    pub fn category_id(&mut self, category_id: i64) -> &mut Self {
        self.category_ids.get_or_insert(vec![]).push(category_id);
        self
    }
    pub fn build(&self) -> OfferCardRequest {
        OfferCardRequest {
            offer_ids: self.offer_ids.clone(),
            card_statuses: self.card_statuses.clone(),
            category_ids: self.category_ids.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferCardContentStatusDTO {
    pub offer_cards: Option<Vec<OfferCardDTO>>,
    pub paging: Option<ForwardScrollingPagerDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferCardDTO {
    pub offer_id: String,
    pub mapping: Option<GetMappingDTO>,
    pub card_status: Option<OfferCardStatusType>,
    pub content_rating: Option<i32>,
    pub recommendations: Option<Vec<OfferCardRecommendationDTO>>,
    pub errors: Option<Vec<OfferErrorDTO>>,
    pub warnings: Option<Vec<OfferErrorDTO>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferCardRecommendationDTO {
    #[serde(rename = "type")]
    pub recommendation_type: OfferCardRecommendationType,
    percent: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferErrorDTO {
    pub message: Option<String>,
    pub comment: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferCardRecommendationType {
    HasVideo,
    RecognizedVendor,
    Main,
    Additional,
    Distinctive,
    Filterable,
    PictureCount,
    HasDescription,
    HasBarcode,
    FirstPictureSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCharacteristicsResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<CategoryContentParametersDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryContentParametersDTO {
    pub category_id: i64,
    pub parameters: Option<Vec<CategoryParameterDTO>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryParameterDTO {
    pub id: i64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub parameter_type: ParameterType,
    pub description: Option<String>,
    pub recommendation_types: Option<Vec<OfferCardRecommendationType>>,
    pub required: bool,
    pub filtering: bool,
    pub distinctive: bool,
    pub multivalue: bool,
    pub allow_custom_values: bool,
    pub values: Option<Vec<ParameterValueOptionDTO>>,
    pub constraints: Option<ParameterValueConstraintsDTO>,
    pub value_restrictions: Option<Vec<ValueRestrictionDTO>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ParameterType {
    Text,
    Enum,
    Boolean,
    Numeric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValueOptionDTO {
    pub id: i64,
    pub value: String,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterValueConstraintsDTO {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub max_length: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRestrictionDTO {
    pub limiting_parameter_id: i64,
    pub limited_values: Vec<OptionValuesLimitedDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionValuesLimitedDTO {
    pub limiting_option_value_id: i64,
    pub option_value_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOfferCardsRequest {
    pub offers_content: Vec<OfferContentDTO>,
}
/// Товар с указанными характеристиками.
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferContentDTO {
    pub offer_id: String,
    pub category_id: i64,
    pub parameter_values: Vec<ParameterValueDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterValueDTO {
    pub parameter_id: i64,
    pub value_id: Option<i64>,
    pub value: String,
}
impl OfferContentDTO {
    pub fn builder() -> OfferContentDTOBuilder<NoOfferId, NoCategoryId> {
        OfferContentDTOBuilder::default()
    }
}
#[derive(Default)]
pub struct NoCategoryId;
#[derive(Default)]
pub struct NoOfferId;
pub struct WithOfferId(String);
pub struct WithCategoryId(i64);
#[derive(Default)]
pub struct OfferContentDTOBuilder<O, C> {
    offer_id: O,
    category_id: C,
    parameter_values: Vec<ParameterValueDTO>,
}

impl<O, C> OfferContentDTOBuilder<O, C> {
    pub fn offer_id(self, offer_id: impl Into<String>) -> OfferContentDTOBuilder<WithOfferId, C> {
        OfferContentDTOBuilder {
            offer_id: WithOfferId(offer_id.into()),
            category_id: self.category_id,
            parameter_values: self.parameter_values,
        }
    }
    pub fn category_id(self, category_id: i64) -> OfferContentDTOBuilder<O, WithCategoryId> {
        OfferContentDTOBuilder {
            offer_id: self.offer_id,
            category_id: WithCategoryId(category_id),
            parameter_values: self.parameter_values,
        }
    }
    pub fn parameter_value(
        mut self,
        parameter_id: i64,
        value_id: Option<i64>,
        value: impl Into<String>,
    ) -> Self {
        self.parameter_values.push(ParameterValueDTO {
            parameter_id,
            value_id,
            value: value.into(),
        });
        self
    }
}
impl OfferContentDTOBuilder<WithOfferId, WithCategoryId> {
    pub fn build(self) -> OfferContentDTO {
        OfferContentDTO {
            offer_id: self.offer_id.0,
            category_id: self.category_id.0,
            parameter_values: self.parameter_values,
        }
    }
}
