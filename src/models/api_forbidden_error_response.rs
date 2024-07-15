/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// ApiForbiddenErrorResponse : Неверны авторизационные данные, указанные в запросе, или запрещен доступ к запрашиваемому ресурсу.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiForbiddenErrorResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApiResponseStatusType>,
    /// Список ошибок.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::ApiErrorDto>>,
}

impl ApiForbiddenErrorResponse {
    /// Неверны авторизационные данные, указанные в запросе, или запрещен доступ к запрашиваемому ресурсу.
    pub fn new() -> ApiForbiddenErrorResponse {
        ApiForbiddenErrorResponse {
            status: None,
            errors: None,
        }
    }
}
