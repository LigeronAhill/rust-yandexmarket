/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GenerateShowsSalesReportRequest : Данные, необходимые для генерации отчета.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateShowsSalesReportRequest {
    /// Идентификатор бизнеса.  Указывается, если нужно составить отчет по всем магазинам бизнеса. В запросе обязательно должен быть либо `businessID`, либо `campaignId`, но не оба сразу.
    #[serde(rename = "businessId", skip_serializing_if = "Option::is_none")]
    pub business_id: Option<i64>,
    /// Идентификатор кампании.  Указывается, если нужно составить отчет по конкретному магазину. В запросе обязательно должен быть либо `businessID`, либо `campaignId`, но не оба сразу.
    #[serde(rename = "campaignId", skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<i64>,
    /// Начало периода, включительно.
    #[serde(rename = "dateFrom")]
    pub date_from: String,
    /// Конец периода, включительно.
    #[serde(rename = "dateTo")]
    pub date_to: String,
    #[serde(rename = "grouping")]
    pub grouping: crate::models::ShowsSalesGroupingType,
}

impl GenerateShowsSalesReportRequest {
    /// Данные, необходимые для генерации отчета.
    pub fn new(
        date_from: String,
        date_to: String,
        grouping: crate::models::ShowsSalesGroupingType,
    ) -> GenerateShowsSalesReportRequest {
        GenerateShowsSalesReportRequest {
            business_id: None,
            campaign_id: None,
            date_from,
            date_to,
            grouping,
        }
    }
}
