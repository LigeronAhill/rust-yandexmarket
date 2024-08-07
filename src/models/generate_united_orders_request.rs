/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GenerateUnitedOrdersRequest : Данные, необходимые для генерации отчета.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateUnitedOrdersRequest {
    /// Идентификатор бизнеса.
    #[serde(rename = "businessId")]
    pub business_id: i64,
    /// Начало периода, включительно.
    #[serde(rename = "dateFrom")]
    pub date_from: String,
    /// Конец периода, включительно. Максимальный период — 1 год.
    #[serde(rename = "dateTo")]
    pub date_to: String,
    /// Список магазинов, которые нужны в отчете.
    #[serde(rename = "campaignIds", skip_serializing_if = "Option::is_none")]
    pub campaign_ids: Option<Vec<i64>>,
}

impl GenerateUnitedOrdersRequest {
    /// Данные, необходимые для генерации отчета.
    pub fn new(
        business_id: i64,
        date_from: String,
        date_to: String,
    ) -> GenerateUnitedOrdersRequest {
        GenerateUnitedOrdersRequest {
            business_id,
            date_from,
            date_to,
            campaign_ids: None,
        }
    }
}
