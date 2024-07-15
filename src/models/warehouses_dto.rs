/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// WarehousesDto : Информация о складах и группах складов.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WarehousesDto {
    /// Список складов, не входящих в группы.
    #[serde(rename = "warehouses", skip_serializing_if = "Option::is_none")]
    pub warehouses: Option<Vec<crate::models::WarehouseDto>>,
    /// Список групп складов.
    #[serde(rename = "warehouseGroups", skip_serializing_if = "Option::is_none")]
    pub warehouse_groups: Option<Vec<crate::models::WarehouseGroupDto>>,
}

impl WarehousesDto {
    /// Информация о складах и группах складов.
    pub fn new() -> WarehousesDto {
        WarehousesDto {
            warehouses: None,
            warehouse_groups: None,
        }
    }
}