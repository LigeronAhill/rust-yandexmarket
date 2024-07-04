/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// AgeDto : Возраст в заданных единицах измерения.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AgeDto {
    /// Значение.
    #[serde(rename = "value")]
    pub value: f32,
    #[serde(rename = "ageUnit")]
    pub age_unit: crate::models::AgeUnitType,
}

impl AgeDto {
    /// Возраст в заданных единицах измерения.
    pub fn new(value: f32, age_unit: crate::models::AgeUnitType) -> AgeDto {
        AgeDto { value, age_unit }
    }
}
