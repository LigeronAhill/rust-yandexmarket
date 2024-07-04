/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// RefundStatusType : Cтатус возврата денег:  * `STARTED_BY_USER` — создан клиентом из личного кабинета.  * `REFUND_IN_PROGRESS` — ждет решение о возврате денег.  * `REFUNDED` — по возврату проведены все возвратные денежные транзакции.  * `FAILED` — невозможно провести возврат покупателю.  * `WAITING_FOR_DECISION` — ожидает решения.  * `DECISION_MADE` — по возврату принято решение.  * `REFUNDED_WITH_BONUSES` — возврат осуществлен баллами Плюса или промокодом.  * `REFUNDED_BY_SHOP` — магазин сделал самостоятельно возврат денег.  * `COMPLETE_WITHOUT_REFUND` — возврат денег не требуется.  * `CANCELLED` — возврат отменен.

/// Cтатус возврата денег:  * `STARTED_BY_USER` — создан клиентом из личного кабинета.  * `REFUND_IN_PROGRESS` — ждет решение о возврате денег.  * `REFUNDED` — по возврату проведены все возвратные денежные транзакции.  * `FAILED` — невозможно провести возврат покупателю.  * `WAITING_FOR_DECISION` — ожидает решения.  * `DECISION_MADE` — по возврату принято решение.  * `REFUNDED_WITH_BONUSES` — возврат осуществлен баллами Плюса или промокодом.  * `REFUNDED_BY_SHOP` — магазин сделал самостоятельно возврат денег.  * `COMPLETE_WITHOUT_REFUND` — возврат денег не требуется.  * `CANCELLED` — возврат отменен.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RefundStatusType {
    #[serde(rename = "STARTED_BY_USER")]
    StartedByUser,
    #[serde(rename = "REFUND_IN_PROGRESS")]
    RefundInProgress,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "WAITING_FOR_DECISION")]
    WaitingForDecision,
    #[serde(rename = "DECISION_MADE")]
    DecisionMade,
    #[serde(rename = "REFUNDED_WITH_BONUSES")]
    RefundedWithBonuses,
    #[serde(rename = "REFUNDED_BY_SHOP")]
    RefundedByShop,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "COMPLETE_WITHOUT_REFUND")]
    CompleteWithoutRefund,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for RefundStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::StartedByUser => String::from("STARTED_BY_USER"),
            Self::RefundInProgress => String::from("REFUND_IN_PROGRESS"),
            Self::Refunded => String::from("REFUNDED"),
            Self::Failed => String::from("FAILED"),
            Self::WaitingForDecision => String::from("WAITING_FOR_DECISION"),
            Self::DecisionMade => String::from("DECISION_MADE"),
            Self::RefundedWithBonuses => String::from("REFUNDED_WITH_BONUSES"),
            Self::RefundedByShop => String::from("REFUNDED_BY_SHOP"),
            Self::Cancelled => String::from("CANCELLED"),
            Self::CompleteWithoutRefund => String::from("COMPLETE_WITHOUT_REFUND"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for RefundStatusType {
    fn default() -> RefundStatusType {
        Self::StartedByUser
    }
}
