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

/// OfferCampaignStatusType : Статус товара:  * `PUBLISHED` — Готов к продаже. * `CHECKING` — На проверке. * `DISABLED_BY_PARTNER` — Скрыт вами. * `REJECTED_BY_MARKET` — Отклонен. * `DISABLED_AUTOMATICALLY` — Исправьте ошибки. * `CREATING_CARD` — Создается карточка. * `NO_CARD` — Нужна карточка. * `NO_STOCKS` — Нет на складе.  [Что обозначает каждый из статусов](https://yandex.ru/support/marketplace/assortment/add/statuses.html)

/// Статус товара:  * `PUBLISHED` — Готов к продаже. * `CHECKING` — На проверке. * `DISABLED_BY_PARTNER` — Скрыт вами. * `REJECTED_BY_MARKET` — Отклонен. * `DISABLED_AUTOMATICALLY` — Исправьте ошибки. * `CREATING_CARD` — Создается карточка. * `NO_CARD` — Нужна карточка. * `NO_STOCKS` — Нет на складе.  [Что обозначает каждый из статусов](https://yandex.ru/support/marketplace/assortment/add/statuses.html)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OfferCampaignStatusType {
    #[serde(rename = "PUBLISHED")]
    Published,
    #[serde(rename = "CHECKING")]
    Checking,
    #[serde(rename = "DISABLED_BY_PARTNER")]
    DisabledByPartner,
    #[serde(rename = "DISABLED_AUTOMATICALLY")]
    DisabledAutomatically,
    #[serde(rename = "REJECTED_BY_MARKET")]
    RejectedByMarket,
    #[serde(rename = "CREATING_CARD")]
    CreatingCard,
    #[serde(rename = "NO_CARD")]
    NoCard,
    #[serde(rename = "NO_STOCKS")]
    NoStocks,
}

impl Display for OfferCampaignStatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Published => String::from("PUBLISHED"),
            Self::Checking => String::from("CHECKING"),
            Self::DisabledByPartner => String::from("DISABLED_BY_PARTNER"),
            Self::DisabledAutomatically => String::from("DISABLED_AUTOMATICALLY"),
            Self::RejectedByMarket => String::from("REJECTED_BY_MARKET"),
            Self::CreatingCard => String::from("CREATING_CARD"),
            Self::NoCard => String::from("NO_CARD"),
            Self::NoStocks => String::from("NO_STOCKS"),
        };
        write!(f, "{}", str)
    }
}

impl Default for OfferCampaignStatusType {
    fn default() -> OfferCampaignStatusType {
        Self::Published
    }
}
