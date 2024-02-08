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
