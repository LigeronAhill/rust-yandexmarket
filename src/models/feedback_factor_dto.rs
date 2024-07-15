/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedbackFactorDto : Информация об оценках по параметрам, указанных в отзыве.  При создании отзыва автору предлагается поставить оценки магазину по нескольким параметрам: например, за скорость обработки заказа или удобство самовывоза. Набор параметров зависит от того, какой способ покупки (параметр `delivery`) указал автор.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedbackFactorDto {
    /// Идентификатор параметра.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Название параметра. Например, `Скорость обработки заказа`.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Описание параметра. Например, `Как быстро с вами связались для подтверждения заказа?`.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Оценка по параметру, указанная в отзыве: от `1` (низшая оценка) до `5` (высшая оценка).
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl FeedbackFactorDto {
    /// Информация об оценках по параметрам, указанных в отзыве.  При создании отзыва автору предлагается поставить оценки магазину по нескольким параметрам: например, за скорость обработки заказа или удобство самовывоза. Набор параметров зависит от того, какой способ покупки (параметр `delivery`) указал автор.
    pub fn new() -> FeedbackFactorDto {
        FeedbackFactorDto {
            id: None,
            title: None,
            description: None,
            value: None,
        }
    }
}
