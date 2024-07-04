/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderItemInstanceModificationDto : Позиция в корзине, требующая маркировки.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderItemInstanceModificationDto {
    /// Идентификатор товара в заказе.  Он приходит в ответе на запрос [GET campaigns/{campaignId}/orders/{orderId}](../../reference/orders/getOrder.md) и в запросе Маркета [POST order/accept](../../pushapi/reference/orderAccept.md) — параметр `id` в `items`.
    #[serde(rename = "id")]
    pub id: i64,
    /// Список кодов маркировки единиц товара.
    #[serde(rename = "instances")]
    pub instances: Vec<crate::models::BriefOrderItemInstanceDto>,
}

impl OrderItemInstanceModificationDto {
    /// Позиция в корзине, требующая маркировки.
    pub fn new(
        id: i64,
        instances: Vec<crate::models::BriefOrderItemInstanceDto>,
    ) -> OrderItemInstanceModificationDto {
        OrderItemInstanceModificationDto { id, instances }
    }
}
