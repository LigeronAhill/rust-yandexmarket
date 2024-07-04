/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferCardRecommendationDto : Рекомендация по заполнению карточки товара.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferCardRecommendationDto {
    #[serde(rename = "type")]
    pub r#type: crate::models::OfferCardRecommendationType,
    /// Процент выполнения рекомендации. Указывается для рекомендаций некоторых типов.
    #[serde(rename = "percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
}

impl OfferCardRecommendationDto {
    /// Рекомендация по заполнению карточки товара.
    pub fn new(r#type: crate::models::OfferCardRecommendationType) -> OfferCardRecommendationDto {
        OfferCardRecommendationDto {
            r#type,
            percent: None,
        }
    }
}
