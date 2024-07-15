/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOfferMappingsRequest {
    /// Идентификаторы товаров, информация о которых нужна.  {% note warning \"Такой список возвращается только целиком\" %}  Если вы запрашиваете информацию по конкретным SKU, не заполняйте: * `page_token`; * `limit`; * `cardStatuses`; * `categoryIds`; * `vendorNames`; * `tags`; * `archived`.  {% endnote %}
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
    /// Фильтр по нахождению в архиве.  Передайте `true`, чтобы получить товары, находящиеся в архиве. Если фильтр не заполнен или передано `false`, в ответе возвращаются товары, не находящиеся в архиве.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl GetOfferMappingsRequest {
    pub fn new() -> GetOfferMappingsRequest {
        GetOfferMappingsRequest {
            offer_ids: None,
            card_statuses: None,
            category_ids: None,
            vendor_names: None,
            tags: None,
            archived: None,
        }
    }
}
