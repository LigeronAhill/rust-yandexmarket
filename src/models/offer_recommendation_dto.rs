/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferRecommendationDto : Информация о состоянии цен и рекомендации.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferRecommendationDto {
    #[serde(rename = "offer", skip_serializing_if = "Option::is_none")]
    pub offer: Option<Box<crate::models::OfferForRecommendationDto>>,
    #[serde(rename = "recommendation", skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Box<crate::models::OfferRecommendationInfoDto>>,
}

impl OfferRecommendationDto {
    /// Информация о состоянии цен и рекомендации.
    pub fn new() -> OfferRecommendationDto {
        OfferRecommendationDto {
            offer: None,
            recommendation: None,
        }
    }
}