use crate::models::{FlippingPagerDTO, ScrollingPagerDTO};
use crate::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{geobases::RegionDTO, ApiResponseStatusType};

/// Информация об условиях доставки для данной точки продаж.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutletDeliveryRuleDTO {
    /// Стоимость самовывоза из точки продаж.
    pub cost: i64,
    /// Минимальный срок доставки товаров в точку продаж. Указан в рабочих днях.
    /// Минимальное значение: `0` — доставка в день заказа. Максимальное значение: `60`.
    /// Допустимые сроки доставки (разница между `minDeliveryDays` и `maxDeliveryDays`)
    /// зависят от региона. Для доставки по своему региону разница не должна превышать двух дней.
    /// Например, если `minDeliveryDays` равно 1, то для `maxDeliveryDays` допускаются значения от 1 до 3.
    /// Для доставки в другие регионы:
    ///
    /// - Если `minDeliveryDays` до 18 дней, разница не должна превышать четырех дней.
    /// Например, если `minDeliveryDays` равно 10, то для `maxDeliveryDays` допускаются значения от 10 до 14.
    /// - Если `minDeliveryDays` больше 18 дней, разница должна быть не больше чем в два раза.
    /// Например, если `minDeliveryDays` равно 21, то для `maxDeliveryDays` допускаются значения
    /// от 21 до 42. Обязательный параметр, если `type="DEPOT"` или `type="MIXED"`. Взаимоисключающий
    /// с параметром `unspecifiedDeliveryInterval`.
    pub min_delivery_days: Option<i64>,
    /// Максимальный срок доставки товаров в точку продаж. Указан в рабочих днях. Минимальное значение:
    /// `0` — доставка в день заказа. Максимальное значение: `60`. Допустимые сроки доставки
    /// (разница между `minDeliveryDays` и `maxDeliveryDays`) зависят от региона. Для доставки по своему
    /// региону разница не должна превышать двух дней. Например, если `minDeliveryDays` равно 1, то для
    /// `maxDeliveryDays` допускаются значения от 1 до 3. Для доставки в другие регионы:
    ///
    /// - Если `minDeliveryDays` до 18 дней, разница не должна превышать четырех дней. Например,
    /// если `minDeliveryDays` равно 10, то для `maxDeliveryDays` допускаются значения от 10 до 14.
    /// - Если `minDeliveryDays` больше 18 дней, разница должна быть не больше чем в два раза.
    /// Например, если `minDeliveryDays` равно 21, то для `maxDeliveryDays` допускаются значения
    /// от 21 до 42. Обязательный параметр, если `type="DEPOT"` или `type="MIXED"`. Взаимоисключающий
    /// с параметром `unspecifiedDeliveryInterval`.
    pub max_delivery_days: Option<i64>,
    /// Идентификатор службы доставки товаров в точку продаж. Информацию о службе доставки можно
    /// получить с помощью запроса GET delivery/services.
    pub delivery_service_id: Option<i64>,
    /// Час, до которого покупателю нужно сделать заказ, чтобы он был доставлен в точку продаж в сроки
    /// от `minDeliveryDays` до `maxDeliveryDays`. Если покупатель оформит заказ после указанного часа,
    /// он будет доставлен в сроки от `minDeliveryDays` + 1 рабочий день до `maxDeliveryDays` + 1 рабочий день.
    /// Значение по умолчанию: `24`.
    pub order_before: Option<i64>,
    /// Цена товара, начиная с которой действует бесплатный самовывоз товара из точки продаж.
    pub price_free_pickup: Option<i64>,
    /// Признак доставки товаров в точку продаж на заказ. Признак выставлен, если:
    ///
    /// - точный срок доставки в точку продаж заранее неизвестен (например, если магазин собирает несколько
    /// заказов для отправки в точку или населенный пункт);
    /// - все товары изготавливаются или поставляются на заказ. Возможные значения:
    /// - `true` — товары доставляются в точку продаж на заказ. Параметр указывается только со значением `true`.
    /// Взаимоисключающий с параметрами `minDeliveryDays` и `maxDeliveryDays`.
    pub unspecified_delivery_interval: bool,
}
/// Расписание работы точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutletWorkingScheduleItemDTO {
    /// Точка продаж работает с указанного дня недели
    pub start_day: DayOfWeekType,
    /// Точка продаж работает до указанного дня недели
    pub end_day: DayOfWeekType,
    /// Точка продаж работает c указанного часа. Формат: `ЧЧ:ММ`.
    pub start_time: String,
    /// Точка продаж работает до указанного часа. Формат: `ЧЧ:ММ`.
    pub end_time: String,
}
/// Список режимов работы точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OutletWorkingScheduleDTO {
    /// Признак, работает ли точка продаж в дни государственных праздников. Возможные значения:
    ///
    /// - `false` — точка продаж не работает в дни государственных праздников.
    /// - `true` — точка продаж работает в дни государственных праздников.
    pub work_in_holiday: bool,
    /// Список расписаний работы точки продаж.
    pub schedule_items: Vec<OutletWorkingScheduleItemDTO>,
}
/// Адрес точки продаж.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutletAddressDTO {
    /// Идентификатор региона. Идентификатор можно получить c помощью запроса GET regions.
    ///
    /// Внимание!
    ///
    /// При создании и редактировании точек продаж можно указывать только регионы типов TOWN (город),
    /// CITY (крупный город) и REPUBLIC_AREA (район субъекта федерации). Тип региона указан в выходных
    /// параметрах type запросов GET regions и GET regions/{regionId}.
    #[serde(rename = "regionId")]
    pub region_id: i64,
    /// Улица
    pub street: Option<String>,
    /// Номер дома
    pub number: Option<String>,
    /// Номер строения
    pub building: Option<String>,
    /// Номер владения
    pub estate: Option<String>,
    /// Номер корпуса
    pub block: Option<String>,
    /// Дополнительная информация
    pub additional: Option<String>,
    /// Порядковый номер километра дороги, на котором располагается точка продаж, если отсутствует улица.
    pub km: Option<i64>,
    /// Внимание
    ///
    /// Параметр устарел и не рекомендуется к использованию. Город или населенный пункт возвращается
    /// в параметре `regionId`.
    pub city: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutletResponse {
    pub outlet: FullOutletDTO,
}
/// Информация о точке продаж.
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
///     let outlets = client.outlets().get_all_outlets().await?;
///     let id = outlets.first().unwrap().id;
///     let outlet = client.outlets().get_outlet(id).await?;
///     println!("{outlet:#?}");
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullOutletDTO {
    /// Название точки продаж.
    pub name: String,
    /// Тип точки продаж
    pub r#type: OutletType,
    /// Координаты точки продаж.
    ///
    /// Формат: долгота, широта. Разделители: запятая и / или пробел. Например, `20.4522144, 54.7104264`.
    ///
    /// Если параметр не передан, координаты будут определены по значениям параметров, вложенных в address.
    pub coords: String,
    /// Признак основной точки продаж. Возможные значения:
    ///
    /// - `false` — неосновная точка продаж.
    /// - `true` — основная точка продаж.
    pub is_main: bool,
    /// Идентификатор точки продаж, присвоенный магазином.
    pub shop_outlet_code: String,
    /// Состояние точки продаж. Возможные значения:
    ///
    /// - `HIDDEN` — точка продаж выключена.
    /// - `VISIBLE` — точка продаж включена.
    /// Enum: HIDDEN, VISIBLE, UNKNOWN
    pub visibility: OutletVisibilityType,
    /// Адрес точки продаж.
    pub address: OutletAddressDTO,
    /// Номера телефонов точки продаж. Передавайте в формате: `+7 (999) 999-99-99`.
    pub phones: Vec<String>,
    /// Список режимов работы точки продаж.
    pub working_schedule: OutletWorkingScheduleDTO,
    /// Информация об условиях доставки для данной точки продаж. Обязательный параметр,
    /// если параметр `type=DEPOT` или `type=MIXED`.
    pub delivery_rules: Vec<OutletDeliveryRuleDTO>,
    /// Адрес электронной почты точки продаж. Может содержать только один параметр email.
    /// Адрес электронной почты точки продаж. Допускается любой адрес электронной почты,
    /// соответствующий стандарту RFC 2822. Выводится в виде строки.
    pub emails: Vec<String>,
    /// Срок хранения заказа в собственном пункте выдачи заказов. Считается в днях.
    pub storage_period: i64,
    /// Идентификатор точки продаж, присвоенный Яндекс Маркетом.
    pub id: i64,
    /// Статус точки продаж. Возможные значения:
    ///
    /// - `AT_MODERATION` — проверяется.
    /// - `FAILED` — не прошла проверку и отклонена модератором.
    /// - `MODERATED` — проверена и одобрена.
    /// - `NONMODERATED` — новая точка, нуждается в проверке.
    /// Enum: AT_MODERATION, FAILED, MODERATED, NONMODERATED, UNKNOWN
    pub status: OutletStatusType,
    /// Регион доставки.
    pub region: RegionDTO,
    /// Идентификатор точки продаж, заданный магазином.
    pub shop_outlet_id: String,
    /// Рабочее время.
    pub working_time: String,
    /// Статус модерации.
    pub moderation_reason: Option<String>,
}
/// День недели
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DayOfWeekType {
    /// Понедельник
    Monday,
    /// Вторник
    Tuesday,
    /// Среда
    Wednesday,
    /// Четверг
    Thursday,
    /// Пятница
    Friday,
    /// Суббота
    Saturday,
    /// Воскресенье
    Sunday,
}
/// Состояние точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OutletVisibilityType {
    /// Точка продаж выключена.
    Hidden,
    /// Точка продаж включена.
    Visible,
    Unknown,
}
/// Статус точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutletStatusType {
    /// Проверяется
    AtModeration,
    /// Не прошла проверку и отклонена модератором
    Failed,
    /// Проверена и одобрена
    Moderated,
    /// Новая точка, нуждается в проверке
    Nonmoderated,
    Unknown,
}
/// Тип точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutletType {
    /// Пункт выдачи заказов.
    Depot,
    /// Смешанный тип точки продаж (торговый зал и пункт выдачи заказов).
    Mixed,
    /// Розничная точка продаж (торговый зал).
    Retail,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutletsResponse {
    /// Информация о точках продаж.
    pub outlets: Vec<FullOutletDTO>,
    /// Информация о страницах результатов.
    pub paging: ScrollingPagerDTO,
    /// Модель для пагинации.
    pub pager: FlippingPagerDTO,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseResponse {
    /// Тип ответа.
    pub status: ApiResponseStatusType,
    /// Ответ на запрос информации о лицензиях для точек продаж.
    pub result: OutletLicensesResponseDTO,
}
/// Ответ на запрос информации о лицензиях для точек продаж.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutletLicensesResponseDTO {
    /// Список лицензий.
    pub licenses: Vec<FullOutletLicenseDTO>,
}
/// Информация о лицензии
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullOutletLicenseDTO {
    /// Идентификатор лицензии.
    /// Параметр указывается, только если нужно изменить информацию о существующей лицензии. Ее идентификатор можно узнать с помощью запроса GET campaigns/{campaignId}/outlets/licenses. При передаче информации о новой лицензии указывать идентификатор не нужно.
    /// Идентификатор лицензии присваивается Маркетом. Не путайте его с номером, указанным на лицензии: он передается в параметре number.
    pub id: i64,
    /// Идентификатор точки продаж, для которой действительна лицензия.
    pub outlet_id: i64,
    /// Тип лицензии:
    ///
    /// `ALCOHOL` — лицензия на розничную продажу алкогольной продукции.
    /// Enum: `ALCOHOL`, `UNKNOWN`
    pub license_type: LicenseType,
    /// Номер лицензии.
    pub number: String,
    /// Дата выдачи лицензии.
    /// Формат даты: ISO 8601 со смещением относительно UTC. Нужно передать дату, указанную на лицензии, время 00:00:00 и часовой пояс, соответствующий региону точки продаж. Например, если лицензия для точки продаж в Москве выдана 13 ноября 2017 года, то параметр должен иметь значение `2017-11-13T00:00:00+03:00`.
    /// Не может быть позже даты окончания срока действия, указанной в параметре dateOfExpiry.
    pub date_of_issue: NaiveDateTime,
    /// Дата окончания действия лицензии.
    /// Формат даты: ISO 8601 со смещением относительно UTC. Нужно передать дату, указанную на лицензии, время 00:00:00 и часовой пояс, соответствующий региону точки продаж. Например, если действие лицензии для точки продаж в Москве заканчивается 20 ноября 2022 года, то параметр должен иметь значение 2022-11-20T00:00:00+03:00.
    ///
    /// Не может быть раньше даты выдачи, указанной в параметре dateOfIssue.
    pub date_of_expiry: NaiveDateTime,
    /// Статус проверки лицензии:
    ///
    /// - `NEW` — лицензия проверяется.
    /// - `SUCCESS` — лицензия прошла проверку.
    /// - `FAIL` — лицензия не прошла проверку.
    /// Enum: NEW, SUCCESS, FAIL, REVOKE, DONT_WANT, FAIL_MANUAL
    pub check_status: LicenseCheckStatusType,
    /// Причина, по которой лицензия не прошла проверку. Параметр возвращается, только если параметр checkStatus имеет значение `FAIL`.
    pub check_comment: Option<String>,
}
/// Тип лицензии
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LicenseType {
    Alcohol,
}
/// Статус проверки лицензии
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LicenseCheckStatusType {
    New,
    Success,
    Fail,
}

//-------------------OUT-------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutletResponse {
    pub status: ApiResponseStatusType,
    pub result: OutletResponseDTO,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutletResponseDTO {
    pub id: i64,
}
/// Информация об условиях доставки для данной точки продаж.
///
/// # Example
///
/// ```rust
///
/// use rust_yandexmarket::{MarketClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let outlets = client.outlets().get_all_outlets().await?;
///     let id = outlets.first().unwrap().id;
///     let _outlet = client.outlets().get_outlet(id).await?;
///     let address = rust_yandexmarket::Address::builder()
///         .region_id(13)
///         .street("улица Ленина")
///         .number("69")
///         .block("5")
///         .additional("Вход со двора")
///         .build();
///     let schedule_item_1 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Monday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Friday)
///         .start_time("09:00")
///         .end_time("21:00")
///         .build();
///     let schedule_item_2 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Saturday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Sunday)
///         .start_time("10:00")
///         .end_time("18:00")
///         .build();
///     let delivery_rule = rust_yandexmarket::DeliveryRule::builder()
///         .cost(0)
///         .min_delivery_days(5)
///         .max_delivery_days(7)
///         .order_before(15)
///         .build()?;
///     let outlet_to_create = rust_yandexmarket::Outlet::builder()
///         .name("Test Outlet")
///         .outlet_type(rust_yandexmarket::OutletType::Retail)
///         .coords("20.45, 54.71")
///         .is_main(false)
///         .shop_outlet_code("42")
///         .visibility(rust_yandexmarket::OutletVisibilityType::Hidden)
///         .address(address)
///         .phone("+7 (999) 696-69-69")
///         .phone("+7 (888) 999-66-99")
///         .phones(vec![
///             "+7 (678) 321-65-49".to_string(),
///             "+7 (987) 654-32-11".to_string(),
///         ])
///         .work_in_holiday(true)
///         .schedule_item(schedule_item_1)
///         .schedule_item(schedule_item_2)
///         .delivery_rule(delivery_rule)
///         .email("most@wanted.man")
///         .storage_period(3)
///         .build();
///     // let created = client.outlets().create_outlet(outlet_to_create).await?;
///     // let c = client.outlets().get_outlet(created).await?;
///     // println!("{c:#?}");
///     // let _deleted = client.outlets().delete_outlet(created).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRule;
impl DeliveryRule {
    pub fn builder() -> OutletDeliveryRuleDTOBuilder {
        OutletDeliveryRuleDTOBuilder::default()
    }
}
/// Список режимов работы точки продаж.
///
/// # Example
///
/// ```rust
///
/// use rust_yandexmarket::{MarketClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let outlets = client.outlets().get_all_outlets().await?;
///     let id = outlets.first().unwrap().id;
///     let _outlet = client.outlets().get_outlet(id).await?;
///     let address = rust_yandexmarket::Address::builder()
///         .region_id(13)
///         .street("улица Ленина")
///         .number("69")
///         .block("5")
///         .additional("Вход со двора")
///         .build();
///     let schedule_item_1 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Monday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Friday)
///         .start_time("09:00")
///         .end_time("21:00")
///         .build();
///     let schedule_item_2 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Saturday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Sunday)
///         .start_time("10:00")
///         .end_time("18:00")
///         .build();
///     let delivery_rule = rust_yandexmarket::DeliveryRule::builder()
///         .cost(0)
///         .min_delivery_days(5)
///         .max_delivery_days(7)
///         .order_before(15)
///         .build()?;
///     let outlet_to_create = rust_yandexmarket::Outlet::builder()
///         .name("Test Outlet")
///         .outlet_type(rust_yandexmarket::OutletType::Retail)
///         .coords("20.45, 54.71")
///         .is_main(false)
///         .shop_outlet_code("42")
///         .visibility(rust_yandexmarket::OutletVisibilityType::Hidden)
///         .address(address)
///         .phone("+7 (999) 696-69-69")
///         .phone("+7 (888) 999-66-99")
///         .phones(vec![
///             "+7 (678) 321-65-49".to_string(),
///             "+7 (987) 654-32-11".to_string(),
///         ])
///         .work_in_holiday(true)
///         .schedule_item(schedule_item_1)
///         .schedule_item(schedule_item_2)
///         .delivery_rule(delivery_rule)
///         .email("most@wanted.man")
///         .storage_period(3)
///         .build();
///     // let created = client.outlets().create_outlet(outlet_to_create).await?;
///     // let c = client.outlets().get_outlet(created).await?;
///     // println!("{c:#?}");
///     // let _deleted = client.outlets().delete_outlet(created).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingScheduleItem;
impl WorkingScheduleItem {
    pub fn builder() -> OutletWorkingScheduleItemDTOBuilder {
        OutletWorkingScheduleItemDTOBuilder::default()
    }
}
/// Адрес точки продаж.
///
/// # Example
///
/// ```rust
///
/// use rust_yandexmarket::{MarketClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let outlets = client.outlets().get_all_outlets().await?;
///     let id = outlets.first().unwrap().id;
///     let _outlet = client.outlets().get_outlet(id).await?;
///     let address = rust_yandexmarket::Address::builder()
///         .region_id(13)
///         .street("улица Ленина")
///         .number("69")
///         .block("5")
///         .additional("Вход со двора")
///         .build();
///     let schedule_item_1 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Monday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Friday)
///         .start_time("09:00")
///         .end_time("21:00")
///         .build();
///     let schedule_item_2 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Saturday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Sunday)
///         .start_time("10:00")
///         .end_time("18:00")
///         .build();
///     let delivery_rule = rust_yandexmarket::DeliveryRule::builder()
///         .cost(0)
///         .min_delivery_days(5)
///         .max_delivery_days(7)
///         .order_before(15)
///         .build()?;
///     let outlet_to_create = rust_yandexmarket::Outlet::builder()
///         .name("Test Outlet")
///         .outlet_type(rust_yandexmarket::OutletType::Retail)
///         .coords("20.45, 54.71")
///         .is_main(false)
///         .shop_outlet_code("42")
///         .visibility(rust_yandexmarket::OutletVisibilityType::Hidden)
///         .address(address)
///         .phone("+7 (999) 696-69-69")
///         .phone("+7 (888) 999-66-99")
///         .phones(vec![
///             "+7 (678) 321-65-49".to_string(),
///             "+7 (987) 654-32-11".to_string(),
///         ])
///         .work_in_holiday(true)
///         .schedule_item(schedule_item_1)
///         .schedule_item(schedule_item_2)
///         .delivery_rule(delivery_rule)
///         .email("most@wanted.man")
///         .storage_period(3)
///         .build();
///     // let created = client.outlets().create_outlet(outlet_to_create).await?;
///     // let c = client.outlets().get_outlet(created).await?;
///     // println!("{c:#?}");
///     // let _deleted = client.outlets().delete_outlet(created).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address;
impl Address {
    pub fn builder() -> OutletAddressDTOBuilder<NoRegionId> {
        OutletAddressDTOBuilder::default()
    }
}
/// Информация о точке продаж.
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
///     let outlets = client.outlets().get_all_outlets().await?;
///     let id = outlets.first().unwrap().id;
///     let _outlet = client.outlets().get_outlet(id).await?;
///     let address = rust_yandexmarket::Address::builder()
///         .region_id(13)
///         .street("улица Ленина")
///         .number("69")
///         .block("5")
///         .additional("Вход со двора")
///         .build();
///     let schedule_item_1 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Monday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Friday)
///         .start_time("09:00")
///         .end_time("21:00")
///         .build();
///     let schedule_item_2 = rust_yandexmarket::WorkingScheduleItem::builder()
///         .start_day(rust_yandexmarket::DayOfWeekType::Saturday)
///         .end_day(rust_yandexmarket::DayOfWeekType::Sunday)
///         .start_time("10:00")
///         .end_time("18:00")
///         .build();
///     let delivery_rule = rust_yandexmarket::DeliveryRule::builder()
///         .cost(0)
///         .min_delivery_days(5)
///         .max_delivery_days(7)
///         .order_before(15)
///         .build()?;
///     let outlet_to_create = rust_yandexmarket::Outlet::builder()
///         .name("Test Outlet")
///         .outlet_type(rust_yandexmarket::OutletType::Retail)
///         .coords("20.45, 54.71")
///         .is_main(false)
///         .shop_outlet_code("42")
///         .visibility(rust_yandexmarket::OutletVisibilityType::Hidden)
///         .address(address)
///         .phone("+7 (999) 696-69-69")
///         .phone("+7 (888) 999-66-99")
///         .phones(vec![
///             "+7 (678) 321-65-49".to_string(),
///             "+7 (987) 654-32-11".to_string(),
///         ])
///         .work_in_holiday(true)
///         .schedule_item(schedule_item_1)
///         .schedule_item(schedule_item_2)
///         .delivery_rule(delivery_rule)
///         .email("most@wanted.man")
///         .storage_period(3)
///         .build();
///     // let created = client.outlets().create_outlet(outlet_to_create).await?;
///     // let created_outlet = client.outlets().get_outlet(created).await?;
///     // println!("{created_outlet:#?}");
///     // let mut outlet_to_update = created_outlet;
///     // outlet_to_update.name = "Another name".to_string();
///     // let _ = client
///     //     .outlets()
///     //     .update_outlet(created, outlet_to_update)
///     //     .await?;
///     // let _deleted = client.outlets().delete_outlet(created).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outlet;
impl Outlet {
    pub fn builder(
    ) -> CreateOutletDTOBuilder<NoName, NoOutletType, NoAddress, NoPhones, NoWorkingSchedule> {
        CreateOutletDTOBuilder {
            name: NoName,
            outlet_type: NoOutletType,
            coords: None,
            is_main: None,
            shop_outlet_code: None,
            visibility: None,
            address: NoAddress,
            phones: NoPhones,
            working_schedule: NoWorkingSchedule,
            delivery_rules: None,
            emails: None,
            storage_period: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct CreateOutletDTO {
    name: String,
    #[serde(rename = "type")]
    outlet_type: OutletType,
    coords: Option<String>,
    is_main: Option<bool>,
    shop_outlet_code: Option<String>,
    visibility: Option<OutletVisibilityType>,
    address: OutletAddressDTO,
    phones: Vec<String>,
    working_schedule: OutletWorkingScheduleDTO,
    delivery_rules: Option<Vec<OutletDeliveryRuleDTO>>,
    emails: Option<Vec<String>>,
    storage_period: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithName(String);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithOutletType(OutletType);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithAddress(OutletAddressDTO);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithPhones(Vec<String>);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithWorkingSchedule(OutletWorkingScheduleDTO);
#[derive(Default)]
pub struct NoName;
#[derive(Default)]
pub struct NoOutletType;
#[derive(Default)]
pub struct NoAddress;
#[derive(Default)]
pub struct NoPhones;
#[derive(Default)]
pub struct NoWorkingSchedule;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutletDTOBuilder<N, O, A, P, W> {
    name: N,
    outlet_type: O,
    coords: Option<String>,
    is_main: Option<bool>,
    shop_outlet_code: Option<String>,
    visibility: Option<OutletVisibilityType>,
    address: A,
    phones: P,
    working_schedule: W,
    delivery_rules: Option<Vec<OutletDeliveryRuleDTO>>,
    emails: Option<Vec<String>>,
    storage_period: Option<i64>,
}
impl<N, O, A, P, W> CreateOutletDTOBuilder<N, O, A, P, W> {
    /// Название точки продаж.
    pub fn name(self, name: impl Into<String>) -> CreateOutletDTOBuilder<WithName, O, A, P, W> {
        CreateOutletDTOBuilder {
            name: WithName(name.into()),
            outlet_type: self.outlet_type,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address,
            phones: self.phones,
            working_schedule: self.working_schedule,
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
    /// Тип точки продаж
    pub fn outlet_type(
        self,
        outlet_type: OutletType,
    ) -> CreateOutletDTOBuilder<N, WithOutletType, A, P, W> {
        CreateOutletDTOBuilder {
            name: self.name,
            outlet_type: WithOutletType(outlet_type),
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address,
            phones: self.phones,
            working_schedule: self.working_schedule,
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
    /// Координаты точки продаж.
    ///
    /// Формат: долгота, широта. Разделители: запятая и / или пробел. Например, `20.4522144, 54.7104264`.
    ///
    /// Если параметр не передан, координаты будут определены по значениям параметров, вложенных в address.
    pub fn coords(mut self, coords: impl Into<String>) -> Self {
        let _ = self.coords.insert(coords.into());
        self
    }
    /// Признак основной точки продаж. Возможные значения:
    ///
    /// - `false` — неосновная точка продаж.
    /// - `true` — основная точка продаж.
    pub fn is_main(mut self, is_main: bool) -> Self {
        let _ = self.is_main.insert(is_main);
        self
    }
    /// Идентификатор точки продаж, присвоенный магазином.
    pub fn shop_outlet_code(mut self, code: impl Into<String>) -> Self {
        let _ = self.shop_outlet_code.insert(code.into());
        self
    }
    /// Состояние точки продаж. Возможные значения:
    ///
    /// - `HIDDEN` — точка продаж выключена.
    /// - `VISIBLE` — точка продаж включена.
    /// Enum: HIDDEN, VISIBLE, UNKNOWN
    pub fn visibility(mut self, visibility: OutletVisibilityType) -> Self {
        let _ = self.visibility.insert(visibility);
        self
    }
    /// Адрес точки продаж.
    pub fn address(
        self,
        address: OutletAddressDTO,
    ) -> CreateOutletDTOBuilder<N, O, WithAddress, P, W> {
        CreateOutletDTOBuilder {
            name: self.name,
            outlet_type: self.outlet_type,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: WithAddress(address),
            phones: self.phones,
            working_schedule: self.working_schedule,
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
    /// Номера телефонов точки продаж. Передавайте в формате: `+7 (999) 999-99-99`.
    pub fn phones(self, phones: Vec<String>) -> CreateOutletDTOBuilder<N, O, A, WithPhones, W> {
        CreateOutletDTOBuilder {
            name: self.name,
            outlet_type: self.outlet_type,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address,
            phones: WithPhones(phones),
            working_schedule: self.working_schedule,
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
    /// Информация об условиях доставки для данной точки продаж. Обязательный параметр,
    /// если параметр `type=DEPOT` или `type=MIXED`.
    pub fn delivery_rule(mut self, delivery_rule: OutletDeliveryRuleDTO) -> Self {
        self.delivery_rules
            .get_or_insert(vec![])
            .push(delivery_rule);
        self
    }
    /// Адрес электронной почты точки продаж. Может содержать только один параметр email.
    /// Адрес электронной почты точки продаж. Допускается любой адрес электронной почты,
    /// соответствующий стандарту RFC 2822. Выводится в виде строки.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.emails.get_or_insert(vec![]).push(email.into());
        self
    }
    /// Срок хранения заказа в собственном пункте выдачи заказов. Считается в днях.
    pub fn storage_period(mut self, storage_period: i64) -> Self {
        let _ = self.storage_period.insert(storage_period);
        self
    }
}
impl
    CreateOutletDTOBuilder<WithName, WithOutletType, WithAddress, WithPhones, WithWorkingSchedule>
{
    pub fn build(self) -> CreateOutletDTO {
        CreateOutletDTO {
            name: self.name.0,
            outlet_type: self.outlet_type.0,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address.0,
            phones: self.phones.0,
            working_schedule: self.working_schedule.0,
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
}
#[derive(Default)]
pub struct NoRegionId;
#[derive(Default)]
pub struct WithRegionId(i64);
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OutletAddressDTOBuilder<I> {
    region_id: I,
    street: Option<String>,
    number: Option<String>,
    building: Option<String>,
    estate: Option<String>,
    block: Option<String>,
    additional: Option<String>,
    km: Option<i64>,
    city: Option<String>,
}
impl<A> OutletAddressDTOBuilder<A> {
    /// Идентификатор региона. Идентификатор можно получить c помощью запроса GET regions.
    ///
    /// Внимание!
    ///
    /// При создании и редактировании точек продаж можно указывать только регионы типов TOWN (город),
    /// CITY (крупный город) и REPUBLIC_AREA (район субъекта федерации). Тип региона указан в выходных
    /// параметрах type запросов GET regions и GET regions/{regionId}.
    pub fn region_id(self, region_id: i64) -> OutletAddressDTOBuilder<WithRegionId> {
        OutletAddressDTOBuilder {
            region_id: WithRegionId(region_id),
            street: self.street,
            number: self.number,
            building: self.building,
            estate: self.estate,
            block: self.block,
            additional: self.additional,
            km: self.km,
            city: self.city,
        }
    }
    /// Улица
    pub fn street(mut self, street: impl Into<String>) -> Self {
        let _ = self.street.insert(street.into());
        self
    }
    /// Номер дома
    pub fn number(mut self, number: impl Into<String>) -> Self {
        let _ = self.number.insert(number.into());
        self
    }
    /// Номер строения
    pub fn building(mut self, building: impl Into<String>) -> Self {
        let _ = self.building.insert(building.into());
        self
    }
    /// Номер владения
    pub fn estate(mut self, estate: impl Into<String>) -> Self {
        let _ = self.estate.insert(estate.into());
        self
    }
    /// Номер корпуса
    pub fn block(mut self, block: impl Into<String>) -> Self {
        let _ = self.block.insert(block.into());
        self
    }
    /// Дополнительная информация
    pub fn additional(mut self, additional: impl Into<String>) -> Self {
        let _ = self.additional.insert(additional.into());
        self
    }
    /// Порядковый номер километра дороги, на котором располагается точка продаж, если отсутствует улица.
    pub fn km(mut self, km: i64) -> Self {
        let _ = self.km.insert(km);
        self
    }
    /// Внимание
    ///
    /// Параметр устарел и не рекомендуется к использованию. Город или населенный пункт возвращается
    /// в параметре `regionId`.
    pub fn city(mut self, city: impl Into<String>) -> Self {
        let _ = self.city.insert(city.into());
        self
    }
}
impl OutletAddressDTOBuilder<WithRegionId> {
    pub fn build(self) -> OutletAddressDTO {
        OutletAddressDTO {
            region_id: self.region_id.0,
            street: self.street,
            number: self.number,
            building: self.building,
            estate: self.estate,
            block: self.block,
            additional: self.additional,
            km: self.km,
            city: self.city,
        }
    }
}
impl<N, O, A, W> CreateOutletDTOBuilder<N, O, A, NoPhones, W> {
    /// Номер телефонa точки продаж. Передавайте в формате: `+7 (999) 999-99-99`.
    pub fn phone(self, phone: impl Into<String>) -> CreateOutletDTOBuilder<N, O, A, WithPhones, W> {
        CreateOutletDTOBuilder {
            name: self.name,
            outlet_type: self.outlet_type,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address,
            phones: WithPhones(vec![phone.into()]),
            working_schedule: self.working_schedule,
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
}
impl<N, O, A, W> CreateOutletDTOBuilder<N, O, A, WithPhones, W> {
    /// Номер телефонa точки продаж. Передавайте в формате: `+7 (999) 999-99-99`.
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phones.0.push(phone.into());
        self
    }
}
impl<N, O, A, P> CreateOutletDTOBuilder<N, O, A, P, NoWorkingSchedule> {
    /// Признак, работает ли точка продаж в дни государственных праздников. Возможные значения:
    ///
    /// - `false` — точка продаж не работает в дни государственных праздников.
    /// - `true` — точка продаж работает в дни государственных праздников.
    pub fn work_in_holiday(
        self,
        work_in_holiday: bool,
    ) -> CreateOutletDTOBuilder<N, O, A, P, WithWorkingSchedule> {
        let w = OutletWorkingScheduleDTOBuilder::default()
            .work_in_holiday(work_in_holiday)
            .build();
        CreateOutletDTOBuilder {
            name: self.name,
            outlet_type: self.outlet_type,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address,
            phones: self.phones,
            working_schedule: WithWorkingSchedule(w),
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
    /// Расписание работы точки продаж.
    pub fn schedule_item(
        self,
        schedule_item: OutletWorkingScheduleItemDTO,
    ) -> CreateOutletDTOBuilder<N, O, A, P, WithWorkingSchedule> {
        let w = OutletWorkingScheduleDTOBuilder::default()
            .schedule_item(schedule_item)
            .build();

        CreateOutletDTOBuilder {
            name: self.name,
            outlet_type: self.outlet_type,
            coords: self.coords,
            is_main: self.is_main,
            shop_outlet_code: self.shop_outlet_code,
            visibility: self.visibility,
            address: self.address,
            phones: self.phones,
            working_schedule: WithWorkingSchedule(w),
            delivery_rules: self.delivery_rules,
            emails: self.emails,
            storage_period: self.storage_period,
        }
    }
}
impl<N, O, A, P> CreateOutletDTOBuilder<N, O, A, P, WithWorkingSchedule> {
    /// Признак, работает ли точка продаж в дни государственных праздников. Возможные значения:
    ///
    /// - `false` — точка продаж не работает в дни государственных праздников.
    /// - `true` — точка продаж работает в дни государственных праздников.
    pub fn work_in_holiday(mut self, work_in_holiday: bool) -> Self {
        self.working_schedule.0.work_in_holiday = work_in_holiday;
        self
    }
    /// Расписание работы точки продаж.
    pub fn schedule_item(mut self, item: OutletWorkingScheduleItemDTO) -> Self {
        self.working_schedule.0.schedule_items.push(item);
        self
    }
}
/// Расписание работы точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutletWorkingScheduleItemDTOBuilder {
    start_day: Option<DayOfWeekType>,
    end_day: Option<DayOfWeekType>,
    start_time: Option<String>,
    end_time: Option<String>,
}
impl OutletWorkingScheduleItemDTOBuilder {
    /// Точка продаж работает с указанного дня недели
    pub fn start_day(mut self, start_day: DayOfWeekType) -> Self {
        let _ = self.start_day.insert(start_day);
        self
    }
    /// Точка продаж работает до указанного дня недели
    pub fn end_day(mut self, end_day: DayOfWeekType) -> Self {
        let _ = self.end_day.insert(end_day);
        self
    }
    /// Точка продаж работает c указанного часа. Формат: `ЧЧ:ММ`.
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        let t = start_time.into();
        let time = if check_time(&t) {
            t
        } else {
            String::from("09:00")
        };
        let _ = self.start_time.insert(time);
        self
    }
    /// Точка продаж работает до указанного часа. Формат: `ЧЧ:ММ`.
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        let t = end_time.into();
        let time = if check_time(&t) {
            t
        } else {
            String::from("21:00")
        };
        let _ = self.end_time.insert(time);
        self
    }
    pub fn build(self) -> OutletWorkingScheduleItemDTO {
        OutletWorkingScheduleItemDTO {
            start_day: self.start_day.unwrap_or(DayOfWeekType::Monday),
            end_day: self.end_day.unwrap_or(DayOfWeekType::Sunday),
            start_time: self.start_time.unwrap_or(String::from("09:00")),
            end_time: self.end_time.unwrap_or(String::from("21:00")),
        }
    }
}
/// Список режимов работы точки продаж.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutletWorkingScheduleDTOBuilder {
    work_in_holiday: Option<bool>,
    schedule_items: Option<Vec<OutletWorkingScheduleItemDTO>>,
}
impl OutletWorkingScheduleDTOBuilder {
    /// Признак, работает ли точка продаж в дни государственных праздников. Возможные значения:
    ///
    /// - `false` — точка продаж не работает в дни государственных праздников.
    /// - `true` — точка продаж работает в дни государственных праздников.
    pub fn work_in_holiday(mut self, work_in_holiday: bool) -> Self {
        let _ = self.work_in_holiday.insert(work_in_holiday);
        self
    }
    /// Список расписаний работы точки продаж.
    pub fn schedule_item(mut self, schedule_item: OutletWorkingScheduleItemDTO) -> Self {
        self.schedule_items
            .get_or_insert(vec![])
            .push(schedule_item);
        self
    }
    pub fn build(self) -> OutletWorkingScheduleDTO {
        OutletWorkingScheduleDTO {
            work_in_holiday: self.work_in_holiday.unwrap_or(false),
            schedule_items: self.schedule_items.unwrap_or_default(),
        }
    }
}
fn check_time(time: &String) -> bool {
    // "HH:MM"
    let raw: String = time.into();
    let s = raw.split(':').collect::<Vec<&str>>();
    if s.len() != 2 {
        false
    } else {
        let Some(hours) = s.first() else {
            return false;
        };
        let Some(mins) = s.last() else {
            return false;
        };
        let Ok(h) = hours.parse::<i32>() else {
            return false;
        };
        let Ok(m) = mins.parse::<i32>() else {
            return false;
        };
        (0..24).contains(&h) && (0..60).contains(&m)
    }
}
/// Информация об условиях доставки для данной точки продаж.
#[derive(Default)]
pub struct OutletDeliveryRuleDTOBuilder {
    cost: Option<i64>,
    min_delivery_days: Option<i64>,
    max_delivery_days: Option<i64>,
    delivery_service_id: Option<i64>,
    order_before: Option<i64>,
    price_free_pickup: Option<i64>,
    unspecified_delivery_interval: Option<bool>,
}
impl OutletDeliveryRuleDTOBuilder {
    /// Стоимость самовывоза из точки продаж.
    pub fn cost(mut self, cost: i64) -> Self {
        let _ = self.cost.insert(cost);
        self
    }
    /// Минимальный срок доставки товаров в точку продаж. Указан в рабочих днях.
    /// Минимальное значение: `0` — доставка в день заказа. Максимальное значение: `60`.
    /// Допустимые сроки доставки (разница между `minDeliveryDays` и `maxDeliveryDays`)
    /// зависят от региона. Для доставки по своему региону разница не должна превышать двух дней.
    /// Например, если `minDeliveryDays` равно 1, то для `maxDeliveryDays` допускаются значения от 1 до 3.
    /// Для доставки в другие регионы:
    ///
    /// - Если `minDeliveryDays` до 18 дней, разница не должна превышать четырех дней.
    /// Например, если `minDeliveryDays` равно 10, то для `maxDeliveryDays` допускаются значения от 10 до 14.
    /// - Если `minDeliveryDays` больше 18 дней, разница должна быть не больше чем в два раза.
    /// Например, если `minDeliveryDays` равно 21, то для `maxDeliveryDays` допускаются значения
    /// от 21 до 42. Обязательный параметр, если `type="DEPOT"` или `type="MIXED"`. Взаимоисключающий
    /// с параметром `unspecifiedDeliveryInterval`.
    pub fn min_delivery_days(mut self, min_delivery_days: i64) -> Self {
        let _ = self.min_delivery_days.insert(min_delivery_days);
        self
    }
    /// Максимальный срок доставки товаров в точку продаж. Указан в рабочих днях. Минимальное значение:
    /// `0` — доставка в день заказа. Максимальное значение: `60`. Допустимые сроки доставки
    /// (разница между `minDeliveryDays` и `maxDeliveryDays`) зависят от региона. Для доставки по своему
    /// региону разница не должна превышать двух дней. Например, если `minDeliveryDays` равно 1, то для
    /// `maxDeliveryDays` допускаются значения от 1 до 3. Для доставки в другие регионы:
    ///
    /// - Если `minDeliveryDays` до 18 дней, разница не должна превышать четырех дней. Например,
    /// если `minDeliveryDays` равно 10, то для `maxDeliveryDays` допускаются значения от 10 до 14.
    /// - Если `minDeliveryDays` больше 18 дней, разница должна быть не больше чем в два раза.
    /// Например, если `minDeliveryDays` равно 21, то для `maxDeliveryDays` допускаются значения
    /// от 21 до 42. Обязательный параметр, если `type="DEPOT"` или `type="MIXED"`. Взаимоисключающий
    /// с параметром `unspecifiedDeliveryInterval`.
    pub fn max_delivery_days(mut self, max_delivery_days: i64) -> Self {
        let _ = self.max_delivery_days.insert(max_delivery_days);
        self
    }
    /// Идентификатор службы доставки товаров в точку продаж. Информацию о службе доставки можно
    /// получить с помощью запроса GET delivery/services.
    pub fn delivery_service_id(mut self, delivery_service_id: i64) -> Self {
        let _ = self.delivery_service_id.insert(delivery_service_id);
        self
    }
    /// Час, до которого покупателю нужно сделать заказ, чтобы он был доставлен в точку продаж в сроки
    /// от `minDeliveryDays` до `maxDeliveryDays`. Если покупатель оформит заказ после указанного часа,
    /// он будет доставлен в сроки от `minDeliveryDays` + 1 рабочий день до `maxDeliveryDays` + 1 рабочий день.
    /// Значение по умолчанию: `24`.
    pub fn order_before(mut self, order_before: i64) -> Self {
        let _ = self.order_before.insert(order_before);
        self
    }
    /// Цена товара, начиная с которой действует бесплатный самовывоз товара из точки продаж.
    pub fn price_free_pickup(mut self, price_free_pickup: i64) -> Self {
        let _ = self.price_free_pickup.insert(price_free_pickup);
        self
    }
    /// Признак доставки товаров в точку продаж на заказ. Признак выставлен, если:
    ///
    /// - точный срок доставки в точку продаж заранее неизвестен (например, если магазин собирает несколько
    /// заказов для отправки в точку или населенный пункт);
    /// - все товары изготавливаются или поставляются на заказ. Возможные значения:
    /// - `true` — товары доставляются в точку продаж на заказ. Параметр указывается только со значением `true`.
    /// Взаимоисключающий с параметрами `minDeliveryDays` и `maxDeliveryDays`.
    pub fn unspecified_delivery_interval(mut self, unspecified_delivery_interval: bool) -> Self {
        let _ = self
            .unspecified_delivery_interval
            .insert(unspecified_delivery_interval);
        self
    }
    pub fn build(self) -> Result<OutletDeliveryRuleDTO> {
        let Some(cost) = self.cost else {
            return Err("Cost required!".into());
        };

        Ok(OutletDeliveryRuleDTO {
            cost,
            min_delivery_days: self.min_delivery_days,
            max_delivery_days: self.max_delivery_days,
            delivery_service_id: self.delivery_service_id,
            order_before: self.order_before,
            price_free_pickup: self.price_free_pickup,
            unspecified_delivery_interval: self.unspecified_delivery_interval.unwrap_or_default(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::check_time;
    #[test]
    fn test_check_time() {
        let right_time = "10:00".to_string();
        let middle_time = "25:61".to_string();
        let wrong_time = "waagh".to_string();
        assert!(check_time(&right_time));
        assert!(!check_time(&middle_time));
        assert!(!check_time(&wrong_time))
    }
}
