/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GenerateGoodsTurnoverRequest : Данные, необходимые для генерации отчета.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateGoodsTurnoverRequest {
    /// Идентификатор кампании.
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
}

impl GenerateGoodsTurnoverRequest {
    /// Данные, необходимые для генерации отчета.
    pub fn new(campaign_id: i64) -> GenerateGoodsTurnoverRequest {
        GenerateGoodsTurnoverRequest { campaign_id }
    }
}
