/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderCourierDto : Информация о курьере.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderCourierDto {
    /// Полное имя курьера.
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Номер телефона курьера.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Добавочный номер телефона.
    #[serde(rename = "phoneExtension", skip_serializing_if = "Option::is_none")]
    pub phone_extension: Option<String>,
    /// Номер транспортного средства.
    #[serde(rename = "vehicleNumber", skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,
    /// Описание машины. Например, модель и цвет.
    #[serde(rename = "vehicleDescription", skip_serializing_if = "Option::is_none")]
    pub vehicle_description: Option<String>,
}

impl OrderCourierDto {
    /// Информация о курьере.
    pub fn new() -> OrderCourierDto {
        OrderCourierDto {
            full_name: None,
            phone: None,
            phone_extension: None,
            vehicle_number: None,
            vehicle_description: None,
        }
    }
}
