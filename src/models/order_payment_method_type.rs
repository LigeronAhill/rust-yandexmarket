/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderPaymentMethodType : Способ оплаты заказа:  * Значения, если выбрана оплата при оформлении заказа (`\"paymentType\": \"PREPAID\"`):    * `YANDEX` — банковской картой.    * `APPLE_PAY` — Apple Pay.    * `GOOGLE_PAY` — Google Pay.    * `CREDIT` — в кредит.    * `TINKOFF_CREDIT` — в кредит в Тинькофф Банке.    * `TINKOFF_INSTALLMENTS` — рассрочка в Тинькофф Банке.    * `EXTERNAL_CERTIFICATE` — подарочным сертификатом (например, из приложения «Сбербанк Онлайн»).    * `SBP` — через систему быстрых платежей.    * `B2B_ACCOUNT_PREPAYMENT` — заказ оплачивает организация.   * Значения, если выбрана оплата при получении заказа (`\"paymentType\": \"POSTPAID\"`):    * `CARD_ON_DELIVERY` — банковской картой.    * `CASH_ON_DELIVERY` — наличными.    * `B2B_ACCOUNT_POSTPAYMENT` — заказ оплачивает организация после доставки.  * `UNKNOWN` — неизвестный тип.  Значение по умолчанию: `CASH_ON_DELIVERY`.

/// Способ оплаты заказа:  * Значения, если выбрана оплата при оформлении заказа (`\"paymentType\": \"PREPAID\"`):    * `YANDEX` — банковской картой.    * `APPLE_PAY` — Apple Pay.    * `GOOGLE_PAY` — Google Pay.    * `CREDIT` — в кредит.    * `TINKOFF_CREDIT` — в кредит в Тинькофф Банке.    * `TINKOFF_INSTALLMENTS` — рассрочка в Тинькофф Банке.    * `EXTERNAL_CERTIFICATE` — подарочным сертификатом (например, из приложения «Сбербанк Онлайн»).    * `SBP` — через систему быстрых платежей.    * `B2B_ACCOUNT_PREPAYMENT` — заказ оплачивает организация.   * Значения, если выбрана оплата при получении заказа (`\"paymentType\": \"POSTPAID\"`):    * `CARD_ON_DELIVERY` — банковской картой.    * `CASH_ON_DELIVERY` — наличными.    * `B2B_ACCOUNT_POSTPAYMENT` — заказ оплачивает организация после доставки.  * `UNKNOWN` — неизвестный тип.  Значение по умолчанию: `CASH_ON_DELIVERY`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderPaymentMethodType {
    #[serde(rename = "CASH_ON_DELIVERY")]
    CashOnDelivery,
    #[serde(rename = "CARD_ON_DELIVERY")]
    CardOnDelivery,
    #[serde(rename = "YANDEX")]
    Yandex,
    #[serde(rename = "APPLE_PAY")]
    ApplePay,
    #[serde(rename = "EXTERNAL_CERTIFICATE")]
    ExternalCertificate,
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "GOOGLE_PAY")]
    GooglePay,
    #[serde(rename = "TINKOFF_CREDIT")]
    TinkoffCredit,
    #[serde(rename = "SBP")]
    Sbp,
    #[serde(rename = "TINKOFF_INSTALLMENTS")]
    TinkoffInstallments,
    #[serde(rename = "B2B_ACCOUNT_PREPAYMENT")]
    B2BAccountPrepayment,
    #[serde(rename = "B2B_ACCOUNT_POSTPAYMENT")]
    B2BAccountPostpayment,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for OrderPaymentMethodType {
    fn to_string(&self) -> String {
        match self {
            Self::CashOnDelivery => String::from("CASH_ON_DELIVERY"),
            Self::CardOnDelivery => String::from("CARD_ON_DELIVERY"),
            Self::Yandex => String::from("YANDEX"),
            Self::ApplePay => String::from("APPLE_PAY"),
            Self::ExternalCertificate => String::from("EXTERNAL_CERTIFICATE"),
            Self::Credit => String::from("CREDIT"),
            Self::GooglePay => String::from("GOOGLE_PAY"),
            Self::TinkoffCredit => String::from("TINKOFF_CREDIT"),
            Self::Sbp => String::from("SBP"),
            Self::TinkoffInstallments => String::from("TINKOFF_INSTALLMENTS"),
            Self::B2BAccountPrepayment => String::from("B2B_ACCOUNT_PREPAYMENT"),
            Self::B2BAccountPostpayment => String::from("B2B_ACCOUNT_POSTPAYMENT"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for OrderPaymentMethodType {
    fn default() -> OrderPaymentMethodType {
        Self::CashOnDelivery
    }
}
