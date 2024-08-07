/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// BaseShipmentDto : Информация об отгрузке.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseShipmentDto {
    /// Идентификатор отгрузки.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Начало планового интервала отгрузки.
    #[serde(rename = "planIntervalFrom", skip_serializing_if = "Option::is_none")]
    pub plan_interval_from: Option<String>,
    /// Конец планового интервала отгрузки.
    #[serde(rename = "planIntervalTo", skip_serializing_if = "Option::is_none")]
    pub plan_interval_to: Option<String>,
    #[serde(rename = "shipmentType", skip_serializing_if = "Option::is_none")]
    pub shipment_type: Option<crate::models::ShipmentType>,
    #[serde(rename = "warehouse", skip_serializing_if = "Option::is_none")]
    pub warehouse: Option<Box<crate::models::PartnerShipmentWarehouseDto>>,
    #[serde(rename = "warehouseTo", skip_serializing_if = "Option::is_none")]
    pub warehouse_to: Option<Box<crate::models::PartnerShipmentWarehouseDto>>,
    /// Идентификатор отгрузки в вашей системе. Если вы еще не передавали идентификатор, вернется идентификатор из параметра `id`.
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "deliveryService", skip_serializing_if = "Option::is_none")]
    pub delivery_service: Option<Box<crate::models::DeliveryServiceDto>>,
    #[serde(rename = "palletsCount", skip_serializing_if = "Option::is_none")]
    pub pallets_count: Option<Box<crate::models::PalletsCountDto>>,
    /// Идентификаторы заказов в отгрузке.
    #[serde(rename = "orderIds", skip_serializing_if = "Option::is_none")]
    pub order_ids: Option<Vec<i64>>,
    /// Количество заказов, запланированных к отгрузке.
    #[serde(rename = "draftCount", skip_serializing_if = "Option::is_none")]
    pub draft_count: Option<i32>,
    /// Количество отгруженных заказов.
    #[serde(rename = "plannedCount", skip_serializing_if = "Option::is_none")]
    pub planned_count: Option<i32>,
    /// Количество заказов, принятых в сортировочном центре или пункте приема.
    #[serde(rename = "factCount", skip_serializing_if = "Option::is_none")]
    pub fact_count: Option<i32>,
}

impl BaseShipmentDto {
    /// Информация об отгрузке.
    pub fn new() -> BaseShipmentDto {
        BaseShipmentDto {
            id: None,
            plan_interval_from: None,
            plan_interval_to: None,
            shipment_type: None,
            warehouse: None,
            warehouse_to: None,
            external_id: None,
            delivery_service: None,
            pallets_count: None,
            order_ids: None,
            draft_count: None,
            planned_count: None,
            fact_count: None,
        }
    }
}
