/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// TariffDto : Информация о тарифах, по которым нужно заплатить за услуги Маркета.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TariffDto {
    #[serde(rename = "type")]
    pub r#type: crate::models::TariffType,
    /// {% note warning \"\" %}  Этот параметр устарел. Не используйте его.  {% endnote %}  Значение тарифа в процентах.
    #[serde(rename = "percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<f32>,
    /// Значение тарифа в рублях.
    #[serde(rename = "amount")]
    pub amount: f32,
    /// Параметры расчета тарифа.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::TariffParameterDto>>,
}

impl TariffDto {
    /// Информация о тарифах, по которым нужно заплатить за услуги Маркета.
    pub fn new(r#type: crate::models::TariffType, amount: f32) -> TariffDto {
        TariffDto {
            r#type,
            percent: None,
            amount,
            parameters: None,
        }
    }
}
