/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderStatusType : Статус заказа:  * `CANCELLED` — отменен.  * `DELIVERED` — получен покупателем.  * `DELIVERY` — передан в службу доставки.  * `PICKUP` — доставлен в пункт самовывоза.  * `PROCESSING` — находится в обработке.  * `PENDING` — ожидает обработки со стороны продавца.  * `UNPAID` — оформлен, но еще не оплачен (если выбрана оплата при оформлении).  * `PLACING` — оформляется, подготовка к резервированию.  * `RESERVED` — зарезервирован, но недооформлен.  * `PARTIALLY_RETURNED` — возвращен частично.  * `RETURNED` — возвращен полностью.  * `UNKNOWN` — неизвестный статус.  Также могут возвращаться другие значения. Обрабатывать их не требуется.

/// Статус заказа:  * `CANCELLED` — отменен.  * `DELIVERED` — получен покупателем.  * `DELIVERY` — передан в службу доставки.  * `PICKUP` — доставлен в пункт самовывоза.  * `PROCESSING` — находится в обработке.  * `PENDING` — ожидает обработки со стороны продавца.  * `UNPAID` — оформлен, но еще не оплачен (если выбрана оплата при оформлении).  * `PLACING` — оформляется, подготовка к резервированию.  * `RESERVED` — зарезервирован, но недооформлен.  * `PARTIALLY_RETURNED` — возвращен частично.  * `RETURNED` — возвращен полностью.  * `UNKNOWN` — неизвестный статус.  Также могут возвращаться другие значения. Обрабатывать их не требуется.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderStatusType {
    #[serde(rename = "PLACING")]
    Placing,
    #[serde(rename = "RESERVED")]
    Reserved,
    #[serde(rename = "UNPAID")]
    Unpaid,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "DELIVERY")]
    Delivery,
    #[serde(rename = "PICKUP")]
    Pickup,
    #[serde(rename = "DELIVERED")]
    Delivered,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PARTIALLY_RETURNED")]
    PartiallyReturned,
    #[serde(rename = "RETURNED")]
    Returned,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for OrderStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::Placing => String::from("PLACING"),
            Self::Reserved => String::from("RESERVED"),
            Self::Unpaid => String::from("UNPAID"),
            Self::Processing => String::from("PROCESSING"),
            Self::Delivery => String::from("DELIVERY"),
            Self::Pickup => String::from("PICKUP"),
            Self::Delivered => String::from("DELIVERED"),
            Self::Cancelled => String::from("CANCELLED"),
            Self::Pending => String::from("PENDING"),
            Self::PartiallyReturned => String::from("PARTIALLY_RETURNED"),
            Self::Returned => String::from("RETURNED"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for OrderStatusType {
    fn default() -> OrderStatusType {
        Self::Placing
    }
}