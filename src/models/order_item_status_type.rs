/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderItemStatusType : Возвращенный или невыкупленный товар:  * `REJECTED` — невыкупленный.  * `RETURNED` — возвращенный.

/// Возвращенный или невыкупленный товар:  * `REJECTED` — невыкупленный.  * `RETURNED` — возвращенный.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderItemStatusType {
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "RETURNED")]
    Returned,
}

impl ToString for OrderItemStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::Rejected => String::from("REJECTED"),
            Self::Returned => String::from("RETURNED"),
        }
    }
}

impl Default for OrderItemStatusType {
    fn default() -> OrderItemStatusType {
        Self::Rejected
    }
}
