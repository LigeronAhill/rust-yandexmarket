/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FulfillmentWarehousesDto : Список складов Маркета (FBY).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FulfillmentWarehousesDto {
    /// Список складов Маркета (FBY).
    #[serde(rename = "warehouses", skip_serializing_if = "Option::is_none")]
    pub warehouses: Option<Vec<crate::models::FulfillmentWarehouseDto>>,
}

impl FulfillmentWarehousesDto {
    /// Список складов Маркета (FBY).
    pub fn new() -> FulfillmentWarehousesDto {
        FulfillmentWarehousesDto { warehouses: None }
    }
}
