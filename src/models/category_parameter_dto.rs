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

/// CategoryParameterDto : Характеристика товара.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryParameterDto {
    /// Идентификатор характеристики.
    #[serde(rename = "id")]
    pub id: i64,
    /// Название характеристики.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::ParameterType,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Box<crate::models::CategoryParameterUnitDto>>,
    /// Описание характеристики.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Перечень возможных рекомендаций по заполнению карточки, к которым относится данная характеристика.
    #[serde(
        rename = "recommendationTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommendation_types: Option<Vec<crate::models::OfferCardRecommendationType>>,
    /// Обязательность характеристики.
    #[serde(rename = "required")]
    pub required: bool,
    /// Используется ли характеристика в фильтре.
    #[serde(rename = "filtering")]
    pub filtering: bool,
    /// Является ли характеристика особенностью варианта.
    #[serde(rename = "distinctive")]
    pub distinctive: bool,
    /// Можно ли передать сразу несколько значений.
    #[serde(rename = "multivalue")]
    pub multivalue: bool,
    /// Можно ли передавать собственное значение, которого нет в списке вариантов Маркета. Только для характеристик типа `ENUM`.
    #[serde(rename = "allowCustomValues")]
    pub allow_custom_values: bool,
    /// Список допустимых значений параметра. Только для характеристик типа `ENUM`.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::ParameterValueOptionDto>>,
    #[serde(rename = "constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Box<crate::models::ParameterValueConstraintsDto>>,
    /// Ограничения на значения, накладываемые другими характеристиками. Только для характеристик типа `ENUM`.
    #[serde(rename = "valueRestrictions", skip_serializing_if = "Option::is_none")]
    pub value_restrictions: Option<Vec<crate::models::ValueRestrictionDto>>,
}

impl CategoryParameterDto {
    /// Характеристика товара.
    pub fn new(
        id: i64,
        r#type: crate::models::ParameterType,
        required: bool,
        filtering: bool,
        distinctive: bool,
        multivalue: bool,
        allow_custom_values: bool,
    ) -> CategoryParameterDto {
        CategoryParameterDto {
            id,
            name: None,
            r#type,
            unit: None,
            description: None,
            recommendation_types: None,
            required,
            filtering,
            distinctive,
            multivalue,
            allow_custom_values,
            values: None,
            constraints: None,
            value_restrictions: None,
        }
    }
}