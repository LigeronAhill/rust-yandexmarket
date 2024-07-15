/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferWeightDimensionsDto : Габариты упаковки и вес товара.  Если товар занимает несколько коробок, перед измерением размеров сложите их компактно.  ![Схема измерения многоместных грузов](../../_images/reference/boxes-measure.png)
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferWeightDimensionsDto {
    /// Длина упаковки в см.
    #[serde(rename = "length")]
    pub length: f32,
    /// Ширина упаковки в см.
    #[serde(rename = "width")]
    pub width: f32,
    /// Высота упаковки в см.
    #[serde(rename = "height")]
    pub height: f32,
    /// Вес товара в кг с учетом упаковки (брутто).
    #[serde(rename = "weight")]
    pub weight: f32,
}

impl OfferWeightDimensionsDto {
    /// Габариты упаковки и вес товара.  Если товар занимает несколько коробок, перед измерением размеров сложите их компактно.  ![Схема измерения многоместных грузов](../../_images/reference/boxes-measure.png)
    pub fn new(length: f32, width: f32, height: f32, weight: f32) -> OfferWeightDimensionsDto {
        OfferWeightDimensionsDto {
            length,
            width,
            height,
            weight,
        }
    }
}
