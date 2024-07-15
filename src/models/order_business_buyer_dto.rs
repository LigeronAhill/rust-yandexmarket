/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderBusinessBuyerDto : Информация о покупателе.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderBusinessBuyerDto {
    /// ИНН.
    #[serde(rename = "inn", skip_serializing_if = "Option::is_none")]
    pub inn: Option<String>,
    /// КПП.
    #[serde(rename = "kpp", skip_serializing_if = "Option::is_none")]
    pub kpp: Option<String>,
    /// Наименование юридического лица.
    #[serde(rename = "organizationName", skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    /// Юридический адрес.
    #[serde(
        rename = "organizationJurAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_jur_address: Option<String>,
}

impl OrderBusinessBuyerDto {
    /// Информация о покупателе.
    pub fn new() -> OrderBusinessBuyerDto {
        OrderBusinessBuyerDto {
            inn: None,
            kpp: None,
            organization_name: None,
            organization_jur_address: None,
        }
    }
}