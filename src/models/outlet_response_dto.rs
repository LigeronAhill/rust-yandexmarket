/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OutletResponseDto : Результат выполнения запроса. Выводится, если `status=\"OK\"`.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutletResponseDto {
    /// Идентификатор точки продаж, присвоенный Маркетом.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl OutletResponseDto {
    /// Результат выполнения запроса. Выводится, если `status=\"OK\"`.
    pub fn new() -> OutletResponseDto {
        OutletResponseDto { id: None }
    }
}
