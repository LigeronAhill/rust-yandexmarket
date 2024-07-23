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

/// AddOffersToArchiveResponse : Результат архивации товаров.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddOffersToArchiveResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApiResponseStatusType>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::AddOffersToArchiveDto>>,
}

impl AddOffersToArchiveResponse {
    /// Результат архивации товаров.
    pub fn new() -> AddOffersToArchiveResponse {
        AddOffersToArchiveResponse {
            status: None,
            result: None,
        }
    }
}
