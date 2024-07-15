/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CampaignSettingsDto : Настройки магазина.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CampaignSettingsDto {
    /// Идентификатор региона, в котором находится магазин.
    #[serde(rename = "countryRegion", skip_serializing_if = "Option::is_none")]
    pub country_region: Option<i64>,
    /// Наименование магазина на Яндекс Маркете. Если наименование отсутствует, значение параметра выводится — `null`.
    #[serde(rename = "shopName", skip_serializing_if = "Option::is_none")]
    pub shop_name: Option<String>,
    /// Признак размещения магазина на сайтах партнеров Яндекс Дистрибуции. Возможные значения: * `false` — магазин не размещен на сайтах партнеров Яндекс Дистрибуции. * `true` — магазин размещен на сайтах партнеров Яндекс Дистрибуции.
    #[serde(rename = "showInContext", skip_serializing_if = "Option::is_none")]
    pub show_in_context: Option<bool>,
    /// Признак показа предложений магазина в рекламном блоке над результатами поиска (cпецразмещение). Возможные значения: * `false` — предложения не показываются в блоке cпецразмещения. * `true` — предложения показываются в блоке cпецразмещения.
    #[serde(rename = "showInPremium", skip_serializing_if = "Option::is_none")]
    pub show_in_premium: Option<bool>,
    /// Признак использования внешней интернет-статистики. Возможные значения: * `false` — внешняя интернет-статистика не используется. * `true` — внешняя интернет-статистика используется.
    #[serde(rename = "useOpenStat", skip_serializing_if = "Option::is_none")]
    pub use_open_stat: Option<bool>,
    #[serde(rename = "localRegion", skip_serializing_if = "Option::is_none")]
    pub local_region: Option<Box<crate::models::CampaignSettingsLocalRegionDto>>,
}

impl CampaignSettingsDto {
    /// Настройки магазина.
    pub fn new() -> CampaignSettingsDto {
        CampaignSettingsDto {
            country_region: None,
            shop_name: None,
            show_in_context: None,
            show_in_premium: None,
            use_open_stat: None,
            local_region: None,
        }
    }
}
