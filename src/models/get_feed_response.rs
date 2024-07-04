/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFeedResponse : Ответ на запрос информации о прайс-листе.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFeedResponse {
    #[serde(rename = "feed", skip_serializing_if = "Option::is_none")]
    pub feed: Option<Box<crate::models::FeedDto>>,
}

impl GetFeedResponse {
    /// Ответ на запрос информации о прайс-листе.
    pub fn new() -> GetFeedResponse {
        GetFeedResponse { feed: None }
    }
}
