/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateTimeDto : Время последнего обновления.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateTimeDto {
    /// Время последнего обновления.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl UpdateTimeDto {
    /// Время последнего обновления.
    pub fn new(updated_at: String) -> UpdateTimeDto {
        UpdateTimeDto { updated_at }
    }
}