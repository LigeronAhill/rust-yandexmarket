/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeliveryServicesDto : Информация о службах доставки.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeliveryServicesDto {
    /// Информация о службе доставки.
    #[serde(rename = "deliveryService", skip_serializing_if = "Option::is_none")]
    pub delivery_service: Option<Vec<crate::models::DeliveryServiceInfoDto>>,
}

impl DeliveryServicesDto {
    /// Информация о службах доставки.
    pub fn new() -> DeliveryServicesDto {
        DeliveryServicesDto {
            delivery_service: None,
        }
    }
}