/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferCardRecommendationType : Рекомендация по дополнению или замене контента.  Часть рекомендаций относятся к **основным параметрам**, которые есть у товаров любых категорий. Другие — к тем **характеристикам**, которые есть у товара потому, что он относится к определенной категории.  **1. Рекомендации, относящиеся к основным параметрам**  Каждая такая рекомендация относится к **единственному параметру**. Чтобы заполнить этот параметр, пользуйтесь запросом [POST businesses/{businessId}/offer-mappings/update](../../reference/business-assortment/updateOfferMappings.md).  Рекомендации по заполнению параметров в `updateOfferMappings`:  * `HAS_VIDEO` — добавьте на карточку видео (параметр `videos`). * `RECOGNIZED_VENDOR` — напишите название производителя так, как его пишет сам производитель (параметр `vendor`). * `PICTURE_COUNT` — добавьте изображения (параметр `pictures`). * `HAS_DESCRIPTION` — заполните описание (параметр `description`). * `HAS_BARCODE` — укажите штрихкод (параметр `barcode`). * `FIRST_PICTURE_SIZE` — замените первое изображение более крупным (параметр `pictures`).  **2. Рекомендации, относящиеся к характеристикам по категориям**  Каждая такая рекомендация предполагает заполнение **одной или нескольких характеристик**. Чтобы узнать, какие именно характеристики нужно заполнить, воспользуйтесь запросом [POST category/{categoryId}/parameters](../../reference/content/getCategoryContentParameters.md). Например, если вы получили рекомендацию `MAIN`, нужно заполнить характеристики, имеющие `MAIN` в массиве `recommendationTypes`.  Рекомендации:  * `MAIN` — заполните ключевые характеристики товара. * `ADDITIONAL` — заполните дополнительные характеристики товара. * `FILTERABLE` — заполните характеристики, используемые в фильтрах. * `DISTINCTIVE` — заполните характеристики, которыми отличаются друг от друга варианты товара.

/// Рекомендация по дополнению или замене контента.  Часть рекомендаций относятся к **основным параметрам**, которые есть у товаров любых категорий. Другие — к тем **характеристикам**, которые есть у товара потому, что он относится к определенной категории.  **1. Рекомендации, относящиеся к основным параметрам**  Каждая такая рекомендация относится к **единственному параметру**. Чтобы заполнить этот параметр, пользуйтесь запросом [POST businesses/{businessId}/offer-mappings/update](../../reference/business-assortment/updateOfferMappings.md).  Рекомендации по заполнению параметров в `updateOfferMappings`:  * `HAS_VIDEO` — добавьте на карточку видео (параметр `videos`). * `RECOGNIZED_VENDOR` — напишите название производителя так, как его пишет сам производитель (параметр `vendor`). * `PICTURE_COUNT` — добавьте изображения (параметр `pictures`). * `HAS_DESCRIPTION` — заполните описание (параметр `description`). * `HAS_BARCODE` — укажите штрихкод (параметр `barcode`). * `FIRST_PICTURE_SIZE` — замените первое изображение более крупным (параметр `pictures`).  **2. Рекомендации, относящиеся к характеристикам по категориям**  Каждая такая рекомендация предполагает заполнение **одной или нескольких характеристик**. Чтобы узнать, какие именно характеристики нужно заполнить, воспользуйтесь запросом [POST category/{categoryId}/parameters](../../reference/content/getCategoryContentParameters.md). Например, если вы получили рекомендацию `MAIN`, нужно заполнить характеристики, имеющие `MAIN` в массиве `recommendationTypes`.  Рекомендации:  * `MAIN` — заполните ключевые характеристики товара. * `ADDITIONAL` — заполните дополнительные характеристики товара. * `FILTERABLE` — заполните характеристики, используемые в фильтрах. * `DISTINCTIVE` — заполните характеристики, которыми отличаются друг от друга варианты товара.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OfferCardRecommendationType {
    #[serde(rename = "HAS_VIDEO")]
    HasVideo,
    #[serde(rename = "RECOGNIZED_VENDOR")]
    RecognizedVendor,
    #[serde(rename = "MAIN")]
    Main,
    #[serde(rename = "ADDITIONAL")]
    Additional,
    #[serde(rename = "DISTINCTIVE")]
    Distinctive,
    #[serde(rename = "FILTERABLE")]
    Filterable,
    #[serde(rename = "PICTURE_COUNT")]
    PictureCount,
    #[serde(rename = "HAS_DESCRIPTION")]
    HasDescription,
    #[serde(rename = "HAS_BARCODE")]
    HasBarcode,
    #[serde(rename = "FIRST_PICTURE_SIZE")]
    FirstPictureSize,
}

impl ToString for OfferCardRecommendationType {
    fn to_string(&self) -> String {
        match self {
            Self::HasVideo => String::from("HAS_VIDEO"),
            Self::RecognizedVendor => String::from("RECOGNIZED_VENDOR"),
            Self::Main => String::from("MAIN"),
            Self::Additional => String::from("ADDITIONAL"),
            Self::Distinctive => String::from("DISTINCTIVE"),
            Self::Filterable => String::from("FILTERABLE"),
            Self::PictureCount => String::from("PICTURE_COUNT"),
            Self::HasDescription => String::from("HAS_DESCRIPTION"),
            Self::HasBarcode => String::from("HAS_BARCODE"),
            Self::FirstPictureSize => String::from("FIRST_PICTURE_SIZE"),
        }
    }
}

impl Default for OfferCardRecommendationType {
    fn default() -> OfferCardRecommendationType {
        Self::HasVideo
    }
}
