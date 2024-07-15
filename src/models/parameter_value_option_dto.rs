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

/// ParameterValueOptionDto : Значение характеристики.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParameterValueOptionDto {
    /// Идентификатор значения.
    #[serde(rename = "id")]
    pub id: i64,
    /// Значение.
    #[serde(rename = "value")]
    pub value: String,
    /// Описание значения.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ParameterValueOptionDto {
    /// Значение характеристики.
    pub fn new(id: i64, value: String) -> ParameterValueOptionDto {
        ParameterValueOptionDto {
            id,
            value,
            description: None,
        }
    }
}
