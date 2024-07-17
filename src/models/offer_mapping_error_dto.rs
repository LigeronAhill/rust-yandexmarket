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

/// OfferMappingErrorDto : Текст ошибки.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferMappingErrorDto {
    #[serde(rename = "type")]
    pub r#type: crate::models::OfferMappingErrorType,
    /// Идентификатор характеристики, с которой связана ошибка.
    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<i64>,
    /// Текст ошибки.
    #[serde(rename = "message")]
    pub message: String,
}

impl OfferMappingErrorDto {
    /// Текст ошибки.
    pub fn new(
        r#type: crate::models::OfferMappingErrorType,
        message: String,
    ) -> OfferMappingErrorDto {
        OfferMappingErrorDto {
            r#type,
            parameter_id: None,
            message,
        }
    }
}
