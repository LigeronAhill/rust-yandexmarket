/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// PriceQuarantineVerdictDto : Причина попадания товара в карантин.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PriceQuarantineVerdictDto {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::PriceQuarantineVerdictType>,
    /// Цена, из-за которой товар попал в карантин, и значения для сравнения. Конкретный набор параметров зависит от типа карантина.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<crate::models::PriceQuarantineVerdictParameterDto>>,
}

impl PriceQuarantineVerdictDto {
    /// Причина попадания товара в карантин.
    pub fn new() -> PriceQuarantineVerdictDto {
        PriceQuarantineVerdictDto {
            r#type: None,
            params: None,
        }
    }
}
