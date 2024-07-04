/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// BasePriceDto : Цена на товар.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BasePriceDto {
    /// Значение.
    #[serde(rename = "value")]
    pub value: f32,
    #[serde(rename = "currencyId")]
    pub currency_id: crate::models::CurrencyType,
}

impl BasePriceDto {
    /// Цена на товар.
    pub fn new(value: f32, currency_id: crate::models::CurrencyType) -> BasePriceDto {
        BasePriceDto { value, currency_id }
    }
}