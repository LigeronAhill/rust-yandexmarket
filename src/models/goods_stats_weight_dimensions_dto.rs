/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GoodsStatsWeightDimensionsDto : Информация о весе и габаритах товара.  Если товар уже привязан к карточке (`marketSku`), в ответе вернутся габариты из карточки Маркета, а не размеры, которые вы передаете.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GoodsStatsWeightDimensionsDto {
    /// Длина товара в сантиметрах.
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f32>,
    /// Ширина товара в сантиметрах.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    /// Высота товара в сантиметрах.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    /// Вес товара в килограммах.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
}

impl GoodsStatsWeightDimensionsDto {
    /// Информация о весе и габаритах товара.  Если товар уже привязан к карточке (`marketSku`), в ответе вернутся габариты из карточки Маркета, а не размеры, которые вы передаете.
    pub fn new() -> GoodsStatsWeightDimensionsDto {
        GoodsStatsWeightDimensionsDto {
            length: None,
            width: None,
            height: None,
            weight: None,
        }
    }
}
