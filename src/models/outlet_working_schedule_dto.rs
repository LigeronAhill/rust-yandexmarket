/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OutletWorkingScheduleDto : Список режимов работы точки продаж.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutletWorkingScheduleDto {
    /// Признак, работает ли точка продаж в дни государственных праздников.  Возможные значения:  * `false` — точка продаж не работает в дни государственных праздников. * `true` — точка продаж работает в дни государственных праздников.
    #[serde(rename = "workInHoliday", skip_serializing_if = "Option::is_none")]
    pub work_in_holiday: Option<bool>,
    /// Список расписаний работы точки продаж.
    #[serde(rename = "scheduleItems")]
    pub schedule_items: Vec<crate::models::OutletWorkingScheduleItemDto>,
}

impl OutletWorkingScheduleDto {
    /// Список режимов работы точки продаж.
    pub fn new(
        schedule_items: Vec<crate::models::OutletWorkingScheduleItemDto>,
    ) -> OutletWorkingScheduleDto {
        OutletWorkingScheduleDto {
            work_in_holiday: None,
            schedule_items,
        }
    }
}
