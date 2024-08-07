/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// QualityRatingDetailsDto : Информация о заказах, которые повлияли на индекс качества.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QualityRatingDetailsDto {
    /// Список заказов, которые повлияли на индекс качества.
    #[serde(rename = "affectedOrders")]
    pub affected_orders: Vec<crate::models::QualityRatingAffectedOrderDto>,
}

impl QualityRatingDetailsDto {
    /// Информация о заказах, которые повлияли на индекс качества.
    pub fn new(
        affected_orders: Vec<crate::models::QualityRatingAffectedOrderDto>,
    ) -> QualityRatingDetailsDto {
        QualityRatingDetailsDto { affected_orders }
    }
}
