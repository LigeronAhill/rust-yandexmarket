/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedIndexLogsErrorDto : Информация об ошибке, произошедшей во время индексации прайс-листа.  Выводится, если во время индексации произошли ошибки (`index-log-record status=ERROR`).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedIndexLogsErrorDto {
    /// HTTP-код ошибки индексации прайс-листа.  Выводится, если `type=DOWNLOAD_HTTP_ERROR`.
    #[serde(rename = "httpStatusCode", skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::FeedIndexLogsErrorType>,
    /// Описание ошибки.  Выводится, если `type=DOWNLOAD_ERROR`.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FeedIndexLogsErrorDto {
    /// Информация об ошибке, произошедшей во время индексации прайс-листа.  Выводится, если во время индексации произошли ошибки (`index-log-record status=ERROR`).
    pub fn new() -> FeedIndexLogsErrorDto {
        FeedIndexLogsErrorDto {
            http_status_code: None,
            r#type: None,
            description: None,
        }
    }
}
