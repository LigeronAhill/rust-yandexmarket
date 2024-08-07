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

/// TimePeriodDto : Временной отрезок с комментарием. Требования к содержанию комментария зависят от контекста использования параметра и указаны в описании поля, которое его содержит.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimePeriodDto {
    /// Продолжительность в указанных единицах.
    #[serde(rename = "timePeriod")]
    pub time_period: i32,
    #[serde(rename = "timeUnit")]
    pub time_unit: crate::models::TimeUnitType,
    /// Комментарий.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl TimePeriodDto {
    /// Временной отрезок с комментарием. Требования к содержанию комментария зависят от контекста использования параметра и указаны в описании поля, которое его содержит.
    pub fn new(time_period: i32, time_unit: crate::models::TimeUnitType) -> TimePeriodDto {
        TimePeriodDto {
            time_period,
            time_unit,
            comment: None,
        }
    }
}
