/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetChatHistoryResponseAllOf {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::ChatMessagesResultDto>>,
}

impl GetChatHistoryResponseAllOf {
    pub fn new() -> GetChatHistoryResponseAllOf {
        GetChatHistoryResponseAllOf { result: None }
    }
}
