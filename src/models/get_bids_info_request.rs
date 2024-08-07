/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetBidsInfoRequest : description.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBidsInfoRequest {
    /// Список товаров, для которых нужно получить значения ставок.  Если список не задан, постранично возвращаются все товары со ставками.  Если список задан, результаты возвращаются одной страницей, а параметры `page_token` и `limit` игнорируются.
    #[serde(rename = "skus", skip_serializing_if = "Option::is_none")]
    pub skus: Option<Vec<String>>,
}

impl GetBidsInfoRequest {
    /// description.
    pub fn new() -> GetBidsInfoRequest {
        GetBidsInfoRequest { skus: None }
    }
}
