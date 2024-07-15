/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderVatType : Ставка налога на добавленную стоимость (НДС) на товар:  * `NO_VAT` — НДС не облагается, используется только для отдельных видов услуг.  * `VAT_0` — НДС 0%. Например, используется при продаже товаров, вывезенных в таможенной процедуре экспорта, или при оказании услуг по международной перевозке товаров.  * `VAT_10` — НДС 10%. Например, используется при реализации отдельных продовольственных и медицинских товаров.  * `VAT_10_110` — НДС 10/110. Расчетная ставка НДС 10%, применяется только при предоплате.  * `VAT_20` — НДС 20%. Основная ставка с 2019 года.  * `VAT_20_120` — НДС 20/120. Расчетная ставка НДС 20%, применяется только при предоплате.  * `VAT_18` — НДС 18%. Основная ставка до 2019 года.  * `VAT_18_118` — НДС 18/118. Ставка использовалась до 1 января 2019 года при предоплате.  * `UNKNOWN_VALUE` — неизвестный тип.  Используется только совместно с параметром `payment-method=YANDEX`.

/// Ставка налога на добавленную стоимость (НДС) на товар:  * `NO_VAT` — НДС не облагается, используется только для отдельных видов услуг.  * `VAT_0` — НДС 0%. Например, используется при продаже товаров, вывезенных в таможенной процедуре экспорта, или при оказании услуг по международной перевозке товаров.  * `VAT_10` — НДС 10%. Например, используется при реализации отдельных продовольственных и медицинских товаров.  * `VAT_10_110` — НДС 10/110. Расчетная ставка НДС 10%, применяется только при предоплате.  * `VAT_20` — НДС 20%. Основная ставка с 2019 года.  * `VAT_20_120` — НДС 20/120. Расчетная ставка НДС 20%, применяется только при предоплате.  * `VAT_18` — НДС 18%. Основная ставка до 2019 года.  * `VAT_18_118` — НДС 18/118. Ставка использовалась до 1 января 2019 года при предоплате.  * `UNKNOWN_VALUE` — неизвестный тип.  Используется только совместно с параметром `payment-method=YANDEX`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderVatType {
    #[serde(rename = "NO_VAT")]
    NoVat,
    #[serde(rename = "VAT_0")]
    Vat0,
    #[serde(rename = "VAT_10")]
    Vat10,
    #[serde(rename = "VAT_10_110")]
    Vat10110,
    #[serde(rename = "VAT_20")]
    Vat20,
    #[serde(rename = "VAT_20_120")]
    Vat20120,
    #[serde(rename = "VAT_18")]
    Vat18,
    #[serde(rename = "VAT_18_118")]
    Vat18118,
    #[serde(rename = "UNKNOWN_VALUE")]
    UnknownValue,
}

impl ToString for OrderVatType {
    fn to_string(&self) -> String {
        match self {
            Self::NoVat => String::from("NO_VAT"),
            Self::Vat0 => String::from("VAT_0"),
            Self::Vat10 => String::from("VAT_10"),
            Self::Vat10110 => String::from("VAT_10_110"),
            Self::Vat20 => String::from("VAT_20"),
            Self::Vat20120 => String::from("VAT_20_120"),
            Self::Vat18 => String::from("VAT_18"),
            Self::Vat18118 => String::from("VAT_18_118"),
            Self::UnknownValue => String::from("UNKNOWN_VALUE"),
        }
    }
}

impl Default for OrderVatType {
    fn default() -> OrderVatType {
        Self::NoVat
    }
}