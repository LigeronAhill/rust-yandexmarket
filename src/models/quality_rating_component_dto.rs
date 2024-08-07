/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// QualityRatingComponentDto : Составляющая индекса качества.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QualityRatingComponentDto {
    /// Значение составляющей в процентах.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "componentType")]
    pub component_type: crate::models::QualityRatingComponentType,
}

impl QualityRatingComponentDto {
    /// Составляющая индекса качества.
    pub fn new(
        value: f64,
        component_type: crate::models::QualityRatingComponentType,
    ) -> QualityRatingComponentDto {
        QualityRatingComponentDto {
            value,
            component_type,
        }
    }
}
