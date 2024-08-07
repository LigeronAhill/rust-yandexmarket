/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetQualityRatingRequest : Запрос информации по индексу качества.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetQualityRatingRequest {
    /// Начало периода.  Формат даты: `ГГГГ‑ММ‑ДД`.  Не может быть раньше 30 дней от текущей даты.
    #[serde(rename = "dateFrom", skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Конец периода.  Формат даты: `ГГГГ‑ММ‑ДД`.  Не может быть позже текущей даты.
    #[serde(rename = "dateTo", skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Список идентификаторов магазинов.
    #[serde(rename = "campaignIds")]
    pub campaign_ids: Vec<i64>,
}

impl GetQualityRatingRequest {
    /// Запрос информации по индексу качества.
    pub fn new(campaign_ids: Vec<i64>) -> GetQualityRatingRequest {
        GetQualityRatingRequest {
            date_from: None,
            date_to: None,
            campaign_ids,
        }
    }
}
