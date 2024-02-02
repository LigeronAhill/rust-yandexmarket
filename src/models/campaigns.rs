use crate::models::FlippingPagerDTO;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

use super::geobases::RegionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginsResponse {
    /// Список логинов
    pub logins: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsResponse {
    /// Настройки магазина
    pub settings: CampaignSettingsDTO,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Настройки магазина.
///
/// # Example
///
/// ```rust
/// use rust_yandexmarket::{MarketClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let campaigns = client.campaigns().get_all_campaigns().await?;
///     let id = campaigns.first().unwrap().id;
///     let settings = client.campaigns().get_settings(id).await?;
///     println!("{settings:#?}");
///     Ok(())
/// }
/// ```
pub struct CampaignSettingsDTO {
    /// Идентификатор региона, в котором находится магазин.
    pub country_region: i64,
    /// Наименование магазина на Яндекс Маркете. Если наименование отсутствует, значение параметра выводится — null.
    pub shop_name: Option<String>,
    /// Признак размещения магазина на сайтах партнеров Яндекс Дистрибуции. Возможные значения:
    /// - `false` — магазин не размещен на сайтах партнеров Яндекс Дистрибуции.
    /// - `true` — магазин размещен на сайтах партнеров Яндекс Дистрибуции.
    pub show_in_context: bool,
    /// Признак показа предложений магазина в рекламном блоке над результатами поиска (Спецразмещение).
    /// Возможные значения:
    /// - `false` — предложения не показываются в блоке Спецразмещения.
    /// - `true` — предложения показываются в блоке Спецразмещения.
    pub show_in_premium: bool,
    /// Признак использования внешней интернет-статистики. Возможные значения:
    /// - `false` — внешняя интернет-статистика не используется.
    /// - `true` — внешняя интернет-статистика используется.
    pub use_open_stat: bool,
    /// Информация о своем регионе магазина.
    pub local_region: CampaignSettingsLocalRegionDTO,
}
/// Информация о своем регионе магазина.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSettingsLocalRegionDTO {
    /// Идентификатор региона.
    pub id: i64,
    /// Название региона.
    pub name: String,
    /// Тип региона.
    pub r#type: RegionType,
    /// Источник информации о расписании работы службы доставки. Возможные значения:
    /// - `WEB` — информация получена из настроек личного кабинета магазина на Яндекс Маркете.
    /// - `YML` — информация получена из прайс-листа магазина.
    pub delivery_options_source: CampaignSettingsScheduleSourceType,
    /// Информация о доставке в своем регионе магазина.
    pub delivery: CampaignSettingsDeliveryDTO,
}
/// Информация о доставке в своем регионе магазина.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignSettingsDeliveryDTO {
    /// Расписание работы службы доставки в своем регионе.
    pub schedule: CampaignSettingsScheduleDTO,
}

/// Расписание работы службы доставки в своем регионе.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSettingsScheduleDTO {
    /// Признак работы службы доставки в государственные праздники. Возможные значения.
    /// - `false` — служба доставки не работает в праздничные дни.
    /// - `true` — служба доставки работает в праздничные дни.
    pub available_on_holidays: bool,
    /// Список дней, в которые служба доставки не работает. Дни магазин указал в личном кабинете на Маркете.
    /// Формат даты: `ДД-ММ-ГГГГ`.
    /// Example: `23-09-2022`
    #[serde(deserialize_with = "deserialize_dates_from_str")]
    pub custom_holidays: Vec<NaiveDate>,
    /// Список выходных и праздничных дней, в которые служба доставки работает. Дни магазин указал в личном кабинете на Маркете.
    /// Формат даты: `ДД-ММ-ГГГГ`.
    /// Example: `23-09-2022`
    #[serde(deserialize_with = "deserialize_dates_from_str")]
    pub custom_working_days: Vec<NaiveDate>,
    /// Период, за который рассчитывается итоговый список нерабочих дней службы доставки.
    pub period: CampaignSettingsTimePeriodDTO,
    /// Итоговый список нерабочих дней службы доставки. Список рассчитывается с учетом выходных, нерабочих дней и государственных праздников. Информацию по ним магазин указывает в личном кабинете на Маркете.
    /// Формат даты: `ДД-ММ-ГГГГ`.
    /// Example: `23-09-2022`
    #[serde(deserialize_with = "deserialize_dates_from_str")]
    pub total_holidays: Vec<NaiveDate>,
    /// Список выходных дней недели и государственных праздников.
    /// Номер дня недели. Возможные значения:
    ///
    /// - `1` — понедельник.
    /// - `2` — вторник.
    /// - `3` — среда.
    /// - `4` — четверг.
    /// - `5` — пятница.
    /// - `6` — суббота.
    /// - `7` — воскресенье. Для формата JSON выводится номер дня в виде числа.
    pub weekly_holidays: Vec<i32>,
}

/// Период, за который рассчитывается итоговый список нерабочих дней службы доставки.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSettingsTimePeriodDTO {
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub from_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub to_date: NaiveDate,
}
/// Источник информации о расписании работы службы доставки
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CampaignSettingsScheduleSourceType {
    /// Информация получена из настроек личного кабинета магазина на Яндекс Маркете
    Web,
    /// Информация получена из прайс-листа магазина
    Yml,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResponse {
    /// Информация о магазине
    pub campaign: CampaignDTO,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignsResponse {
    /// Список с информацией по каждому магазину.
    pub campaigns: Vec<CampaignDTO>,
    /// Модель для пагинации.
    pub pager: FlippingPagerDTO,
}
/// Информация о магазине.
///
/// # Example
///
/// ```rust
/// use rust_yandexmarket::{MarketClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let campaigns = client.campaigns().get_all_campaigns().await?;
///     let id = campaigns.first().unwrap().id;
///     let campaign = client.campaigns().get_campaign(id).await?;
///     let logins = client.campaigns().get_logins(campaign.id).await?;
///     let login = logins.first().unwrap();
///     let login_campaigns = client.campaigns().get_logins_campaigns(login).await?;
///     println!("{login_campaigns:#?}");
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignDTO {
    /// URL магазина.
    pub domain: String,
    /// Идентификатор кампании.
    pub id: i64,
    /// Идентификатор плательщика в Яндекс Балансе
    pub client_id: i64,
    /// Информацию о кабинете.
    pub business: BusinessDTO,
    /// Модель, по которой работает магазин:
    ///
    /// - `FBS` — FBS или Экспресс;
    /// - `FBY` — FBY;
    /// - `DBS` — DBS.
    /// Enum: FBS, FBY, DBS
    pub placement_type: PlacementType,
}
/// Информацию о кабинете.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDTO {
    /// Идентификатор кабинета.
    pub id: i64,
    /// Название бизнеса.
    pub name: String,
}
/// Модель, по которой работает магазин
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlacementType {
    /// FBS или Экспресс
    Fbs,
    /// FBY
    Fby,
    /// DBS
    Dbs,
}

fn deserialize_dates_from_str<'de, D>(deserializer: D) -> Result<Vec<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_strings: Vec<String> = Vec::deserialize(deserializer)?;
    let mut dates = Vec::new();
    for date_str in date_strings {
        let date =
            NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)?;
        dates.push(date);
    }
    Ok(dates)
}
fn deserialize_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
}
