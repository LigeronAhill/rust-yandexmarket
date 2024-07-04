/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrdersStatsPaymentDto : Информация о денежных переводах по заказу.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrdersStatsPaymentDto {
    /// Идентификатор денежного перевода.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Дата денежного перевода.  Формат даты: `ГГГГ-ММ-ДД`.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::OrdersStatsPaymentType>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::OrdersStatsPaymentSourceType>,
    /// Сумма денежного перевода. Значение указывается в рублях независимо от способа денежного перевода. Точность — два знака после запятой.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
    #[serde(rename = "paymentOrder", skip_serializing_if = "Option::is_none")]
    pub payment_order: Option<Box<crate::models::OrdersStatsPaymentOrderDto>>,
}

impl OrdersStatsPaymentDto {
    /// Информация о денежных переводах по заказу.
    pub fn new() -> OrdersStatsPaymentDto {
        OrdersStatsPaymentDto {
            id: None,
            date: None,
            r#type: None,
            source: None,
            total: None,
            payment_order: None,
        }
    }
}