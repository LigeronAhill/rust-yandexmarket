/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};

/// ApiClientDataErrorResponse : Ошибка в данных переданных от клиента.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiClientDataErrorResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApiResponseStatusType>,
    /// Список ошибок.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::ApiErrorDto>>,
}

impl ApiClientDataErrorResponse {
    /// Ошибка в данных переданных от клиента.
    pub fn new() -> ApiClientDataErrorResponse {
        ApiClientDataErrorResponse {
            status: None,
            errors: None,
        }
    }
}
