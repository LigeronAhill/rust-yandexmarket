/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderPaymentType : Тип оплаты заказа:  * `PREPAID` — оплата при оформлении заказа.  * `POSTPAID` — оплата при получении заказа.  * `UNKNOWN` — неизвестный тип.  Если параметр отсутствует, заказ будет оплачен при получении.

/// Тип оплаты заказа:  * `PREPAID` — оплата при оформлении заказа.  * `POSTPAID` — оплата при получении заказа.  * `UNKNOWN` — неизвестный тип.  Если параметр отсутствует, заказ будет оплачен при получении.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderPaymentType {
    #[serde(rename = "PREPAID")]
    Prepaid,
    #[serde(rename = "POSTPAID")]
    Postpaid,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for OrderPaymentType {
    fn to_string(&self) -> String {
        match self {
            Self::Prepaid => String::from("PREPAID"),
            Self::Postpaid => String::from("POSTPAID"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for OrderPaymentType {
    fn default() -> OrderPaymentType {
        Self::Prepaid
    }
}