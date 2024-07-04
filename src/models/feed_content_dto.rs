/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedContentDto : Информация о проверке содержимого прайс-листа, загруженного на Маркет.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedContentDto {
    /// Количество предложений, в которых найдены ошибки на этапе загрузки прайс-листа. Выводится, если параметр `content status=OK`.
    #[serde(
        rename = "rejectedOffersCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub rejected_offers_count: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::FeedStatusType>,
    /// Количество предложений в прайс-листе. Выводится, если параметр `content status=OK`.
    #[serde(rename = "totalOffersCount", skip_serializing_if = "Option::is_none")]
    pub total_offers_count: Option<i64>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::FeedContentErrorDto>>,
}

impl FeedContentDto {
    /// Информация о проверке содержимого прайс-листа, загруженного на Маркет.
    pub fn new() -> FeedContentDto {
        FeedContentDto {
            rejected_offers_count: None,
            status: None,
            total_offers_count: None,
            error: None,
        }
    }
}
