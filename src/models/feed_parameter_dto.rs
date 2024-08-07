/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedParameterDto : Параметр прайс-листа.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedParameterDto {
    /// Удалить ли значение параметра.  Возможное значение: * `true` — удалить значение параметра.  Используется вместе с параметром `name`.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название параметра.  Возможное значение: - `reparseIntervalMinutes` — период скачивания прайс-листа. Маркет будет скачивать прайс-лист через количество минут, указанное в параметре `value`. Например, при `value=1440`, Маркет будет скачивать прайс-лист один раз в сутки.  {% note alert %}  Несмотря на установленное значение, Маркет скачает прайс-лист один раз в сутки.  {% endnote %}  Обязательный параметр.
    #[serde(rename = "name")]
    pub name: String,
    /// Значения параметра.  Используется вместе с параметром `name`.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<i32>>,
}

impl FeedParameterDto {
    /// Параметр прайс-листа.
    pub fn new(name: String) -> FeedParameterDto {
        FeedParameterDto {
            deleted: None,
            name,
            values: None,
        }
    }
}
