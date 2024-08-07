/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedPublicationPriceAndStockUpdateDto : Последнее обновление цен и наличия товаров на Маркете. Если последнее по времени обновление было полным, в параметре выводятся те же данные, что и в параметре `full`. Выводится, если параметр `publication status=OK`.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedPublicationPriceAndStockUpdateDto {
    /// Дата и время, которые магазин указал в прайс-листе.  Формат даты: ISO 8601 со смещением относительно UTC. Например, `2017-11-21T00:42:42+03:00`.
    #[serde(rename = "fileTime", skip_serializing_if = "Option::is_none")]
    pub file_time: Option<String>,
    /// Дата и время публикации предложений из прайс-листа на Маркете.  Формат даты: ISO 8601 со смещением относительно UTC. Например, `2017-11-21T00:42:42+03:00`.
    #[serde(rename = "publishedTime", skip_serializing_if = "Option::is_none")]
    pub published_time: Option<String>,
}

impl FeedPublicationPriceAndStockUpdateDto {
    /// Последнее обновление цен и наличия товаров на Маркете. Если последнее по времени обновление было полным, в параметре выводятся те же данные, что и в параметре `full`. Выводится, если параметр `publication status=OK`.
    pub fn new() -> FeedPublicationPriceAndStockUpdateDto {
        FeedPublicationPriceAndStockUpdateDto {
            file_time: None,
            published_time: None,
        }
    }
}
