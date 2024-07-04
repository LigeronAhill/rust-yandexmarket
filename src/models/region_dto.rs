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

/// RegionDto : Регион доставки.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegionDto {
    /// Идентификатор региона.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Название региона.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::RegionType,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::RegionDto>>,
    /// Дочерние регионы.
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::RegionDto>>,
}

impl RegionDto {
    /// Регион доставки.
    pub fn new(name: String, r#type: crate::models::RegionType) -> RegionDto {
        RegionDto {
            id: None,
            name,
            r#type,
            parent: None,
            children: None,
        }
    }
}
