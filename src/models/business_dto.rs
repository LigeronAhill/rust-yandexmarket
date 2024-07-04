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

/// BusinessDto : Информация о кабинете.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BusinessDto {
    /// Идентификатор кабинета.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Название бизнеса.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BusinessDto {
    /// Информация о кабинете.
    pub fn new() -> BusinessDto {
        BusinessDto {
            id: None,
            name: None,
        }
    }
}