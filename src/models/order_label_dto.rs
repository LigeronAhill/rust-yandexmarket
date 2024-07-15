/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderLabelDto : Данные для печати ярлыка.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderLabelDto {
    /// Идентификатор заказа.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Количество коробок в заказе.
    #[serde(rename = "placesNumber")]
    pub places_number: i32,
    /// URL файла с ярлыками‑наклейками на все коробки в заказе.  Соответствует URL, по которому выполняется запрос [GET campaigns/{campaignId}/orders/{orderId}/delivery/labels](../../reference/orders/generateOrderLabels.md).
    #[serde(rename = "url")]
    pub url: String,
    /// Информация на ярлыке.
    #[serde(rename = "parcelBoxLabels")]
    pub parcel_box_labels: Vec<crate::models::ParcelBoxLabelDto>,
}

impl OrderLabelDto {
    /// Данные для печати ярлыка.
    pub fn new(
        order_id: i64,
        places_number: i32,
        url: String,
        parcel_box_labels: Vec<crate::models::ParcelBoxLabelDto>,
    ) -> OrderLabelDto {
        OrderLabelDto {
            order_id,
            places_number,
            url,
            parcel_box_labels,
        }
    }
}
