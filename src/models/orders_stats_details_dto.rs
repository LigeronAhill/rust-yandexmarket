/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrdersStatsDetailsDto : Информация об удалении товара из заказа.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrdersStatsDetailsDto {
    #[serde(rename = "itemStatus", skip_serializing_if = "Option::is_none")]
    pub item_status: Option<crate::models::OrdersStatsItemStatusType>,
    /// Количество товара со статусом, указанном в параметре `itemStatus`.
    #[serde(rename = "itemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// Дата, когда товар получил статус, указанный в параметре `itemStatus`.  Формат даты: `ГГГГ-ММ-ДД`.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    #[serde(rename = "stockType", skip_serializing_if = "Option::is_none")]
    pub stock_type: Option<crate::models::OrdersStatsStockType>,
}

impl OrdersStatsDetailsDto {
    /// Информация об удалении товара из заказа.
    pub fn new() -> OrdersStatsDetailsDto {
        OrdersStatsDetailsDto {
            item_status: None,
            item_count: None,
            update_date: None,
            stock_type: None,
        }
    }
}
