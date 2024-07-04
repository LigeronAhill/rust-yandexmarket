/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentBoxesDto : В ответе Маркет возвращает переданный вами список грузовых мест. Не обращайте на это поле внимания.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentBoxesDto {
    /// Список грузовых мест. Маркет определил количество мест по длине этого списка.
    #[serde(rename = "boxes", skip_serializing_if = "Option::is_none")]
    pub boxes: Option<Vec<crate::models::ParcelBoxDto>>,
}

impl ShipmentBoxesDto {
    /// В ответе Маркет возвращает переданный вами список грузовых мест. Не обращайте на это поле внимания.
    pub fn new() -> ShipmentBoxesDto {
        ShipmentBoxesDto { boxes: None }
    }
}
