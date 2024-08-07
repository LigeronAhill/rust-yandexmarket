/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// PageFormatType : Размещение ярлыков на странице: * `A7` — в PDF-файле будут страницы формата близкому к A7. На каждой странице размещается ярлык размером 75 × 120 мм (80,4 × 125,6 мм с учетом полей). * `A4` — в PDF-файле будут страницы формата A4. На каждой странице размещаются восемь ярлыков размером 70,6 × 99,1 мм без полей.

/// Размещение ярлыков на странице: * `A7` — в PDF-файле будут страницы формата близкому к A7. На каждой странице размещается ярлык размером 75 × 120 мм (80,4 × 125,6 мм с учетом полей). * `A4` — в PDF-файле будут страницы формата A4. На каждой странице размещаются восемь ярлыков размером 70,6 × 99,1 мм без полей.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PageFormatType {
    #[serde(rename = "A7")]
    A7,
    #[serde(rename = "A4")]
    A4,
}

impl ToString for PageFormatType {
    fn to_string(&self) -> String {
        match self {
            Self::A7 => String::from("A7"),
            Self::A4 => String::from("A4"),
        }
    }
}

impl Default for PageFormatType {
    fn default() -> PageFormatType {
        Self::A7
    }
}
