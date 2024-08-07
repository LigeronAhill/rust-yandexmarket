/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderTrackDto : Информация о трек-номере посылки (DBS).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderTrackDto {
    /// Трек‑номер посылки.
    #[serde(rename = "trackCode", skip_serializing_if = "Option::is_none")]
    pub track_code: Option<String>,
    /// Идентификатор службы доставки. Информацию о службе доставки можно получить с помощью запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md).
    #[serde(rename = "deliveryServiceId", skip_serializing_if = "Option::is_none")]
    pub delivery_service_id: Option<i64>,
}

impl OrderTrackDto {
    /// Информация о трек-номере посылки (DBS).
    pub fn new() -> OrderTrackDto {
        OrderTrackDto {
            track_code: None,
            delivery_service_id: None,
        }
    }
}
