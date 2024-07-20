/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// CalculatedTariffType : Услуга Маркета:  * `AGENCY_COMMISSION` — прием платежа покупателя.  * `PAYMENT_TRANSFER` — перевод платежа покупателя.  * `FEE` — размещение товара на Маркете.  * `DELIVERY_TO_CUSTOMER` — доставка покупателю.  * `CROSSREGIONAL_DELIVERY` — доставка в федеральный округ, город или населенный пункт.  * `EXPRESS_DELIVERY` — экспресс-доставка покупателю.  * `SORTING` — обработка заказа.  * `MIDDLE_MILE` — средняя миля.  Подробнее об услугах Маркета читайте [в Справке Маркета для продавцов](https://yandex.ru/support/marketplace/introduction/rates/index.html).

/// Услуга Маркета:  * `AGENCY_COMMISSION` — прием платежа покупателя.  * `PAYMENT_TRANSFER` — перевод платежа покупателя.  * `FEE` — размещение товара на Маркете.  * `DELIVERY_TO_CUSTOMER` — доставка покупателю.  * `CROSSREGIONAL_DELIVERY` — доставка в федеральный округ, город или населенный пункт.  * `EXPRESS_DELIVERY` — экспресс-доставка покупателю.  * `SORTING` — обработка заказа.  * `MIDDLE_MILE` — средняя миля.  Подробнее об услугах Маркета читайте [в Справке Маркета для продавцов](https://yandex.ru/support/marketplace/introduction/rates/index.html).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalculatedTariffType {
    #[serde(rename = "AGENCY_COMMISSION")]
    AgencyCommission,
    #[serde(rename = "PAYMENT_TRANSFER")]
    PaymentTransfer,
    #[serde(rename = "FEE")]
    Fee,
    #[serde(rename = "DELIVERY_TO_CUSTOMER")]
    DeliveryToCustomer,
    #[serde(rename = "CROSSREGIONAL_DELIVERY")]
    CrossregionalDelivery,
    #[serde(rename = "EXPRESS_DELIVERY")]
    ExpressDelivery,
    #[serde(rename = "SORTING")]
    Sorting,
    #[serde(rename = "MIDDLE_MILE")]
    MiddleMile,
}

impl Display for CalculatedTariffType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::AgencyCommission => String::from("AGENCY_COMMISSION"),
            Self::PaymentTransfer => String::from("PAYMENT_TRANSFER"),
            Self::Fee => String::from("FEE"),
            Self::DeliveryToCustomer => String::from("DELIVERY_TO_CUSTOMER"),
            Self::CrossregionalDelivery => String::from("CROSSREGIONAL_DELIVERY"),
            Self::ExpressDelivery => String::from("EXPRESS_DELIVERY"),
            Self::Sorting => String::from("SORTING"),
            Self::MiddleMile => String::from("MIDDLE_MILE"),
        };
        write!(f, "{}", str)
    }
}

impl Default for CalculatedTariffType {
    fn default() -> CalculatedTariffType {
        Self::AgencyCommission
    }
}
