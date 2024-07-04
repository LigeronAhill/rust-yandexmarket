/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCategoriesMaxSaleQuantumRequest : Список категорий, для которых нужно вернуть лимит на установку кванта и минимального количества товаров.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCategoriesMaxSaleQuantumRequest {
    /// Идентификаторы листовых категории на Маркете — тех, у которых нет дочерних категорий.
    #[serde(rename = "marketCategoryIds")]
    pub market_category_ids: Vec<i64>,
}

impl GetCategoriesMaxSaleQuantumRequest {
    /// Список категорий, для которых нужно вернуть лимит на установку кванта и минимального количества товаров.
    pub fn new(market_category_ids: Vec<i64>) -> GetCategoriesMaxSaleQuantumRequest {
        GetCategoriesMaxSaleQuantumRequest {
            market_category_ids,
        }
    }
}
