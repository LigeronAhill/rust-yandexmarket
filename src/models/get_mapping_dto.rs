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

/// GetMappingDto : Информация о товарах в каталоге.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMappingDto {
    /// SKU на Маркете.
    #[serde(rename = "marketSku", skip_serializing_if = "Option::is_none")]
    pub market_sku: Option<i64>,
    /// Название карточки товара.  Может отсутствовать в ответе, если товар еще не привязан к карточке.
    #[serde(rename = "marketSkuName", skip_serializing_if = "Option::is_none")]
    pub market_sku_name: Option<String>,
    /// Идентификатор модели на Маркете.  Может отсутствовать в ответе, если товар еще не привязан к карточке.
    #[serde(rename = "marketModelId", skip_serializing_if = "Option::is_none")]
    pub market_model_id: Option<i64>,
    /// Название модели на Маркете.  Может отсутствовать в ответе, если товар еще не привязан к карточке.
    #[serde(rename = "marketModelName", skip_serializing_if = "Option::is_none")]
    pub market_model_name: Option<String>,
    /// Идентификатор категории на Маркете, в которую попал товар.  Может отсутствовать в ответе, если Маркет еще не определил категорию товара.
    #[serde(rename = "marketCategoryId", skip_serializing_if = "Option::is_none")]
    pub market_category_id: Option<i64>,
    /// Название категории карточки на Маркете.  Может отсутствовать в ответе, если Маркет еще не определил категорию товара.
    #[serde(rename = "marketCategoryName", skip_serializing_if = "Option::is_none")]
    pub market_category_name: Option<String>,
}

impl GetMappingDto {
    /// Информация о товарах в каталоге.
    pub fn new() -> GetMappingDto {
        GetMappingDto {
            market_sku: None,
            market_sku_name: None,
            market_model_id: None,
            market_model_name: None,
            market_category_id: None,
            market_category_name: None,
        }
    }
}
