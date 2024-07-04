/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CalculatedTariffDto : Информация об услугах Маркета.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CalculatedTariffDto {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::CalculatedTariffType>,
    /// Стоимость услуги в рублях.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Параметры расчета тарифа.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::TariffParameterDto>>,
}

impl CalculatedTariffDto {
    /// Информация об услугах Маркета.
    pub fn new() -> CalculatedTariffDto {
        CalculatedTariffDto {
            r#type: None,
            amount: None,
            parameters: None,
        }
    }
}
