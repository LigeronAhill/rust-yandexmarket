/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedIndexLogsResultDto : Результат выполнения запроса отчета по индексации прайс-листа.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedIndexLogsResultDto {
    #[serde(rename = "feed", skip_serializing_if = "Option::is_none")]
    pub feed: Option<Box<crate::models::FeedIndexLogsFeedDto>>,
    /// Список отчетов по индексации прайс-листа.
    #[serde(rename = "indexLogRecords", skip_serializing_if = "Option::is_none")]
    pub index_log_records: Option<Vec<crate::models::FeedIndexLogsRecordDto>>,
    /// Количество отчетов на всех страницах выходных данных.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl FeedIndexLogsResultDto {
    /// Результат выполнения запроса отчета по индексации прайс-листа.
    pub fn new() -> FeedIndexLogsResultDto {
        FeedIndexLogsResultDto {
            feed: None,
            index_log_records: None,
            total: None,
        }
    }
}
