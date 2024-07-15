/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderDeliveryDateReasonType : Причина переноса доставки заказа. Возможные причины изменения даты:   - ```USER_MOVED_DELIVERY_DATES``` — покупатель попросил изменить дату или вы договорились привезти ему заказ раньше изначальной даты. Кроме этого указывается для подтверждения даты доставки товаров на заказ с долгой (31-60 дней) доставкой.   - ```PARTNER_MOVED_DELIVERY_DATES``` — магазин не может доставить заказ в срок.

/// Причина переноса доставки заказа. Возможные причины изменения даты:   - ```USER_MOVED_DELIVERY_DATES``` — покупатель попросил изменить дату или вы договорились привезти ему заказ раньше изначальной даты. Кроме этого указывается для подтверждения даты доставки товаров на заказ с долгой (31-60 дней) доставкой.   - ```PARTNER_MOVED_DELIVERY_DATES``` — магазин не может доставить заказ в срок.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderDeliveryDateReasonType {
    #[serde(rename = "USER_MOVED_DELIVERY_DATES")]
    UserMovedDeliveryDates,
    #[serde(rename = "PARTNER_MOVED_DELIVERY_DATES")]
    PartnerMovedDeliveryDates,
}

impl ToString for OrderDeliveryDateReasonType {
    fn to_string(&self) -> String {
        match self {
            Self::UserMovedDeliveryDates => String::from("USER_MOVED_DELIVERY_DATES"),
            Self::PartnerMovedDeliveryDates => String::from("PARTNER_MOVED_DELIVERY_DATES"),
        }
    }
}

impl Default for OrderDeliveryDateReasonType {
    fn default() -> OrderDeliveryDateReasonType {
        Self::UserMovedDeliveryDates
    }
}