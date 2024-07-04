/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// BriefOrderItemInstanceDto : Идентификатор единицы товара.  Заполните только одно поле в зависимости от того, в какой системе маркирован товар.  Подробно о работе с маркируемыми товарами рассказано [в Справке Маркета для продавцов](https://yandex.ru/support/marketplace/orders/cz.html).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BriefOrderItemInstanceDto {
    /// Код идентификации единицы товара [в системе «Честный ЗНАК»](https://честныйзнак.рф/).  {% note warning %}  Не экранируйте косую черту в коде символа-разделителя `\\u001d`!  ✅ `01030410947874432155Qbag!\\u001d93Zjqw`  ❌ `01030410947874432155Qbag!\\\\u001d93Zjqw`  Косые черты и кавычки в других местах экранируйте по правилам JSON: `\\\\` и `\\\"`  {% endnote %}
    #[serde(rename = "cis", skip_serializing_if = "Option::is_none")]
    pub cis: Option<String>,
    /// Уникальный идентификационный номер ювелирного изделия.  Представляет собой число из 16 цифр.
    #[serde(rename = "uin", skip_serializing_if = "Option::is_none")]
    pub uin: Option<String>,
    /// Регистрационный номер партии товара.  Представляет собой строку из четырех чисел, разделенных косой чертой: ХХХХХХХХ/ХХХХХХ/ХХХХХХХ/ХХХ.  Первая часть — код таможни, которая зарегистрировала декларацию на партию товара. Далее — дата, номер декларации и номер маркированного товара в декларации.
    #[serde(rename = "rnpt", skip_serializing_if = "Option::is_none")]
    pub rnpt: Option<String>,
    /// Грузовая таможенная декларация.  Представляет собой строку из трех чисел, разделенных косой чертой: ХХХХХХХХ/ХХХХХХ/ХХХХХХХ.  Первая часть — код таможни, которая зарегистрировала декларацию на ввезенные товары. Далее — дата и номер декларации.
    #[serde(rename = "gtd", skip_serializing_if = "Option::is_none")]
    pub gtd: Option<String>,
}

impl BriefOrderItemInstanceDto {
    /// Идентификатор единицы товара.  Заполните только одно поле в зависимости от того, в какой системе маркирован товар.  Подробно о работе с маркируемыми товарами рассказано [в Справке Маркета для продавцов](https://yandex.ru/support/marketplace/orders/cz.html).
    pub fn new() -> BriefOrderItemInstanceDto {
        BriefOrderItemInstanceDto {
            cis: None,
            uin: None,
            rnpt: None,
            gtd: None,
        }
    }
}