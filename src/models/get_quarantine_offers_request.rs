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

/// GetQuarantineOffersRequest : Фильтрации товаров  В запросе можно указать либо фильтр offerIds, либо любые другие фильтры товаров. Совместное использование фильтра offerIds с другими фильтрациями приведет к ошибке.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetQuarantineOffersRequest {
    /// Идентификаторы товаров, информация о которых нужна. <br><br> ⚠️ Не используйте это поле одновременно с фильтрами по статусам карточек, категориям, брендам или тегам. Если вы хотите воспользоваться фильтрами, оставьте поле пустым.
    #[serde(rename = "offerIds", skip_serializing_if = "Option::is_none")]
    pub offer_ids: Option<Vec<String>>,
    /// Фильтр по статусам карточек.  [Что такое карточка товара](https://yandex.ru/support/marketplace/assortment/content/index.html)
    #[serde(rename = "cardStatuses", skip_serializing_if = "Option::is_none")]
    pub card_statuses: Option<Vec<crate::models::OfferCardStatusType>>,
    /// Фильтр по категориям на Маркете.
    #[serde(rename = "categoryIds", skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<Vec<i32>>,
    /// Фильтр по брендам.
    #[serde(rename = "vendorNames", skip_serializing_if = "Option::is_none")]
    pub vendor_names: Option<Vec<String>>,
    /// Фильтр по тегам.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl GetQuarantineOffersRequest {
    /// Фильтрации товаров  В запросе можно указать либо фильтр offerIds, либо любые другие фильтры товаров. Совместное использование фильтра offerIds с другими фильтрациями приведет к ошибке.
    pub fn new() -> GetQuarantineOffersRequest {
        GetQuarantineOffersRequest {
            offer_ids: None,
            card_statuses: None,
            category_ids: None,
            vendor_names: None,
            tags: None,
        }
    }
}
