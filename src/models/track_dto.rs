/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// TrackDto : Информация о трек-номерах.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackDto {
    /// Трек-код почтового отправления.
    #[serde(rename = "trackCode", skip_serializing_if = "Option::is_none")]
    pub track_code: Option<String>,
}

impl TrackDto {
    /// Информация о трек-номерах.
    pub fn new() -> TrackDto {
        TrackDto { track_code: None }
    }
}