/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// ValueRestrictionDto : Ограничение на возможные значения, накладываемое другой характеристикой.  Если ограничивающая характеристика принимает определенное значение, список возможных значений ограничиваемой характеристики сокращается.  **Пример**  Характеристика **размер** сама по себе может принимать девять разных значений: `S`, `M`, `L`, `44`, `46`, `48`, `42/164`, `46/176`, `44S`.  Если ограничивающая характеристика **размерная сетка** принимает значение `RU`, список возможных значений размера сокращается до `44`, `46`, `48`.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValueRestrictionDto {
    /// Идентификатор ограничивающей характеристики.
    #[serde(rename = "limitingParameterId")]
    pub limiting_parameter_id: i64,
    /// Значения ограничивающей характеристики и соответствующие допустимые значения текущей характеристики.
    #[serde(rename = "limitedValues")]
    pub limited_values: Vec<crate::models::OptionValuesLimitedDto>,
}

impl ValueRestrictionDto {
    /// Ограничение на возможные значения, накладываемое другой характеристикой.  Если ограничивающая характеристика принимает определенное значение, список возможных значений ограничиваемой характеристики сокращается.  **Пример**  Характеристика **размер** сама по себе может принимать девять разных значений: `S`, `M`, `L`, `44`, `46`, `48`, `42/164`, `46/176`, `44S`.  Если ограничивающая характеристика **размерная сетка** принимает значение `RU`, список возможных значений размера сокращается до `44`, `46`, `48`.
    pub fn new(
        limiting_parameter_id: i64,
        limited_values: Vec<crate::models::OptionValuesLimitedDto>,
    ) -> ValueRestrictionDto {
        ValueRestrictionDto {
            limiting_parameter_id,
            limited_values,
        }
    }
}
