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

/// UnitDto : Единица измерения.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnitDto {
    /// Идентификатор единицы измерения.
    #[serde(rename = "id")]
    pub id: i64,
    /// Сокращенное название единицы измерения.
    #[serde(rename = "name")]
    pub name: String,
    /// Полное название единицы измерения.
    #[serde(rename = "fullName")]
    pub full_name: String,
}

impl UnitDto {
    /// Единица измерения.
    pub fn new(id: i64, name: String, full_name: String) -> UnitDto {
        UnitDto {
            id,
            name,
            full_name,
        }
    }
}
