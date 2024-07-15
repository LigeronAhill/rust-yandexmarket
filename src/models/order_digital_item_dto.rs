/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderDigitalItemDto : Ключ цифрового товара.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderDigitalItemDto {
    /// Идентификатор товара в заказе.  Он приходит в ответе на запрос [GET campaigns/{campaignId}/orders/{orderId}](../../reference/orders/getOrder.md) и в запросе Маркета [POST order/accept](../../pushapi/reference/orderAccept.md) — параметр `id` в `items`.
    #[serde(rename = "id")]
    pub id: i64,
    /// Сам ключ.
    #[serde(rename = "code")]
    pub code: String,
    /// Инструкция по активации.
    #[serde(rename = "slip")]
    pub slip: String,
    /// Дата, до которой нужно активировать ключ. Если ключ действует бессрочно, укажите любую дату в отдаленном будущем.  Формат даты: `ГГГГ-ММ-ДД`.
    #[serde(rename = "activate_till")]
    pub activate_till: String,
}

impl OrderDigitalItemDto {
    /// Ключ цифрового товара.
    pub fn new(id: i64, code: String, slip: String, activate_till: String) -> OrderDigitalItemDto {
        OrderDigitalItemDto {
            id,
            code,
            slip,
            activate_till,
        }
    }
}