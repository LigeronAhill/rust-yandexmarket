/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GenerateMassOrderLabelsRequest : Данные, необходимые для генерации файла.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateMassOrderLabelsRequest {
    /// Идентификатор кабинета.
    #[serde(rename = "businessId")]
    pub business_id: i64,
    /// Список идентификаторов заказов.
    #[serde(rename = "orderIds")]
    pub order_ids: Vec<i64>,
}

impl GenerateMassOrderLabelsRequest {
    /// Данные, необходимые для генерации файла.
    pub fn new(business_id: i64, order_ids: Vec<i64>) -> GenerateMassOrderLabelsRequest {
        GenerateMassOrderLabelsRequest {
            business_id,
            order_ids,
        }
    }
}
