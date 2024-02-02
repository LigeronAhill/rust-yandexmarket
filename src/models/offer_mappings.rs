use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{ApiResponseStatusType, ScrollingPagerDTO};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct OfferMappingRequest {
    pub offer_ids: Option<Vec<String>>,
    pub card_statuses: Option<Vec<OfferCardStatusType>>,
    pub category_ids: Option<Vec<i64>>,
    pub vendor_names: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub archived: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[skip_serializing_none]
pub enum OfferCardStatusType {
    HasCardCanNotUpdate,
    HasCardCanUpdate,
    HasCardCanUpdateErrors,
    HasCardCanUpdateProcessing,
    NoCardNeedContent,
    NoCardMarketWillCreate,
    NoCardErrors,
    NoCardProcessing,
    NoCardAddToCampaign,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct OfferMappingResponse {
    pub status: ApiResponseStatusType,
    pub result: GetOfferMappingResultDTO,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct GetOfferMappingResultDTO {
    pub paging: ScrollingPagerDTO,
    pub offer_mappings: Vec<GetOfferMappingDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct GetOfferMappingDTO {
    pub offer: GetOfferDTO,
    pub mapping: GetMappingDTO,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct GetMappingDTO {
    /// Идентификатор карточки на Маркете.
    /// Может отсутствовать в ответе, если товар еще не привязан к карточке.
    pub market_sku: Option<i64>,
    /// Название карточки товара.
    /// Может отсутствовать в ответе, если товар еще не привязан к карточке.
    pub market_sku_name: Option<String>,
    /// Идентификатор модели на Маркете.
    /// Может отсутствовать в ответе, если товар еще не привязан к карточке.
    pub market_model_id: Option<i64>,
    /// Название модели на Маркете.
    /// Может отсутствовать в ответе, если товар еще не привязан к карточке.
    pub market_model_name: Option<String>,
    /// Идентификатор категории карточки на Маркете.
    /// Может отсутствовать в ответе, если Маркет еще не определил категорию товара.
    pub market_category_id: Option<i64>,
    /// Название категории карточки на Маркете.
    /// Может отсутствовать в ответе, если Маркет еще не определил категорию товара.
    pub market_category_name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct GetOfferDTO {
    /// Ваш SKU — идентификатор товара в вашей системе.
    /// Разрешена любая последовательность длиной до 80 знаков. В нее могут входить английские и русские буквы, цифры и символы . , / \ ( ) [ ] - = _
    /// Правила использования SKU:
    /// У каждого товара SKU должен быть свой.
    /// SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.
    /// Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.
    pub offer_id: String,
    /// Составляйте название по схеме: тип + бренд или производитель + модель + особенности, если есть (например, цвет, размер или вес) и количество в упаковке.
    /// Не включайте в название условия продажи (например, «скидка», «бесплатная доставка» и т. д.), эмоциональные характеристики («хит», «супер» и т. д.). Не пишите слова большими буквами — кроме устоявшихся названий брендов и моделей.
    /// Оптимальная длина — 50–60 символов, максимальная — 256.
    /// Example: Ударная дрель Makita HP1630, 710 Вт
    pub name: Option<String>,
    /// Категория, к которой магазин относит свой товар. Она помогает точнее определить для товара категорию в каталоге Маркета.
    /// Указывайте конкретные категории — например, набор ножей лучше отнести к категории Столовые приборы, а не просто Посуда.
    /// Выбирайте категории, которые описывают товар, а не абстрактный признак — например, Духи, а не Подарки.
    pub category: Option<String>,
    /// Ссылки на изображения товара. Изображение по первой ссылке считается основным, остальные дополнительными.
    /// Ссылок может быть до 10.
    /// Указывайте ссылку целиком, включая протокол http или https.
    /// Максимальная длина — 512 символов.
    /// Русские буквы в URL можно.
    /// Можно использовать прямые ссылки на изображения и на Яндекс Диск. Ссылки на Яндекс Диске нужно копировать с помощью функции Поделиться. Относительные ссылки и ссылки на другие облачные хранилища — не работают.
    /// ✅ https://example-shop.ru/images/sku12345.jpg
    /// ✅ https://yadi.sk/i/NaBoRsimVOLov
    /// ❌ /images/sku12345.jpg
    /// ❌ https://www.dropbox.com/s/818f/tovar.jpg
    /// Ссылки на изображение должны быть постоянными. Нельзя использовать динамические ссылки, меняющиеся от выгрузки к выгрузке.
    /// Если нужно заменить изображение, выложите новое изображение по новой ссылке, а ссылку на старое удалите. Если просто заменить изображение по старой ссылке, оно не обновится.
    pub pictures: Option<Vec<String>>,
    /// Ссылка (URL) на видео товара.
    /// Внимание
    /// Пока действует временное ограничение: ссылка может быть только одна.
    /// Указывайте ссылку целиком, включая протокол http или https.
    /// Максимальная длина — 512 символов.
    /// Русские буквы в URL можно.
    /// Можно использовать прямые ссылки на видео и на Яндекс Диск. Ссылки на Яндекс Диске нужно копировать с помощью функции Поделиться. Относительные ссылки и ссылки на другие облачные хранилища — не работают.
    /// ✅ https://example-shop.ru/video/sku12345.avi
    /// ✅ https://yadi.sk/i/NaBoRsimVOLov
    /// ❌ /video/sku12345.avi
    /// ❌ https://www.dropbox.com/s/818f/super-tovar.avi
    /// Ссылки на видео должны быть постоянными. Нельзя использовать динамические ссылки, меняющиеся от выгрузки к выгрузке.
    /// Если нужно заменить видео, выложите новое видео по новой ссылке, а ссылку на старое удалите. Если просто заменить видео по старой ссылке, оно не обновится.
    pub videos: Option<Vec<String>>,
    /// Название бренда или производителя. Должно быть записано так, как его пишет сам бренд.
    /// Example: LEVENHUK
    pub vendor: Option<String>,
    /// Указывайте в виде последовательности цифр. Подойдут коды EAN-13, EAN-8, UPC-A, UPC-E или Code 128.
    /// Для книг указывайте ISBN.
    /// Для товаров определенных категорий и торговых марок штрихкод должен быть действительным кодом GTIN. Обратите внимание: внутренние штрихкоды, начинающиеся на 2 или 02, и коды формата Code 128 не являются GTIN.
    /// Example: 46012300000000
    pub barcodes: Option<Vec<String>>,
    /// Подробное описание товара: например, его преимущества и особенности.
    /// Не давайте в описании инструкций по установке и сборке. Не используйте слова «скидка», «распродажа», «дешевый», «подарок» (кроме подарочных категорий), «бесплатно», «акция», «специальная цена», «новинка», «new», «аналог», «заказ», «хит». Не указывайте никакой контактной информации и не давайте ссылок.
    /// Можно использовать теги:
    /// <h>, <h1>, <h2> и так далее — для заголовков;
    /// <br> и <p> — для переноса строки;
    /// <ol> — для нумерованного списка;
    /// <ul> — для маркированного списка;
    /// <li> — для создания элементов списка (должен находиться внутри <ol> или <ul>);
    /// <div> — поддерживается, но не влияет на отображение текста.
    /// Оптимальная длина — 400–600 символов, максимальная — 6000.
    pub description: Option<String>,
    /// Страна, где был произведен товар.
    /// Записывайте названия стран так, как они записаны в списке.
    /// Example: Россия
    pub manufacturer_countries: Option<Vec<String>>,
    /// Габариты упаковки и вес товара.
    pub weight_dimensions: Option<OfferWeightDimensionsDTO>,
    /// Артикул товара от производителя.
    /// Example: VNDR-0005A
    pub vendor_code: Option<String>,
    /// Метки товара, используемые магазином. Покупателям теги не видны. По тегам можно группировать и фильтровать разные товары в каталоге — например, товары одной серии, коллекции или линейки.
    /// Максимальная длина тега 20 символов. У одного товара может быть максимум 10 тегов. Всего можно создать не больше 50 разных тегов.
    /// Example: до 500 рублей
    pub tags: Option<Vec<String>>,
    /// Срок годности — период, по прошествии которого товар становится непригоден.
    /// Указывайте срок, указанный на банке или упаковке. Текущая дата, дата поставки или дата отгрузки значения не имеет.
    /// Обязательно указывайте срок, если он есть.
    /// В комментарии укажите условия хранения. Например, «Хранить в сухом помещении».
    pub shelf_life: Option<TimePeriodDTO>,
    /// Срок службы — период, в течение которого товар должен исправно выполнять свою функцию.
    /// Обязательно указывайте срок, если он есть.
    /// В комментарии укажите условия хранения. Например, «Использовать при температуре не ниже −10 градусов».
    pub life_time: Option<TimePeriodDTO>,
    /// Гарантийный срок — период, в течение которого можно бесплатно заменить или починить товар.
    /// Обязательно указывайте срок, если он есть.
    /// В комментарии опишите особенности гарантийного обслуживания. Например, «Гарантия на аккумулятор — 6 месяцев».
    pub guarantee_period: Option<TimePeriodDTO>,
    /// Код товара в единой Товарной номенклатуре внешнеэкономической деятельности (ТН ВЭД) — 10 или 14 цифр без пробелов.
    /// Обязательно укажите, если он есть.
    /// Example: 8517610008
    pub customs_commodity_code: Option<String>,
    /// Номера документов на товар: сертификата, декларации соответствия и т. п.
    /// Передавать можно только номера документов, сканы которого загружены в личном кабинете продавца по инструкции.
    pub certificates: Option<Vec<String>>,
    /// Количество грузовых мест.
    /// Параметр используется, если товар представляет собой несколько коробок, упаковок и так далее. Например, кондиционер занимает два места — внешний и внутренний блоки в двух коробках.
    /// Для товаров, занимающих одно место, не передавайте этот параметр.
    pub box_count: Option<i32>,
    /// Состояние уцененного товара.
    /// Используется только для товаров, продаваемых с уценкой.
    pub condition: Option<OfferConditionDTO>,
    /// Особый тип товара. Указывается, если товар — книга, аудиокнига, лекарство, музыка, видео или поставляется под заказ.
    /// Enum: DEFAULT, MEDICINE, BOOK, AUDIOBOOK, ARTIST_TITLE, ON_DEMAND
    #[serde(rename = "type")]
    pub offer_type: Option<OfferType>,
    /// Признак цифрового товара. Укажите true, если товар доставляется по электронной почте.
    pub downloadable: Option<bool>,
    /// Параметр включает для товара пометку 18+. Устанавливайте ее только для товаров, которые относятся к удовлетворению сексуальных потребностей.
    pub adult: Option<bool>,
    /// Если товар не предназначен для детей младше определенного возраста, укажите это.
    /// Возрастное ограничение можно задавать в годах (с нуля, с 6, 12, 16 или 18) или в месяцах (любое число от 0 до 12).
    pub age: Option<AgeDTO>,
    /// Характеристики, которые есть только у товаров конкретной категории — например, диаметр колес велосипеда или материал подошвы обуви.
    /// Параметры товара.
    /// Используйте POST businesses/{businessId}/offer-cards/update для передачи характеристик товара, которые специфичны для его категории. Так переданные характеристики с большей вероятностью попадут на карточку.
    pub params: Option<Vec<OfferParamDTO>>,
    /// Цена.
    /// Цена с указанием скидки.
    /// Время последнего обновления.
    pub basic_price: Option<GetPriceWithDiscountDTO>,
    /// Себестоимость — затраты на самостоятельное производство товара или закупку у производителя или поставщиков.
    /// Цена на товар.
    /// Время последнего обновления.
    pub purchase_price: Option<GetPriceDTO>,
    /// Дополнительные расходы на товар. Например, на доставку или упаковку.
    /// Цена на товар.
    /// Время последнего обновления.
    pub additional_expenses: Option<GetPriceDTO>,
    /// Цена для скидок с Маркетом
    /// Маркет может компенсировать до половины скидки. Назначьте минимальную цену до вычета тарифов, по которой готовы продавать товар, а мы рассчитаем скидку и размер софинансирования.
    /// Если Маркет не готов софинансировать скидку, покупатель её не увидит.
    /// Цена на товар.
    /// Время последнего обновления.
    pub cofinance_price: Option<GetPriceDTO>,
    /// Статус карточки товара.
    /// Enum: HAS_CARD_CAN_NOT_UPDATE, HAS_CARD_CAN_UPDATE, HAS_CARD_CAN_UPDATE_ERRORS, HAS_CARD_CAN_UPDATE_PROCESSING, NO_CARD_NEED_CONTENT, NO_CARD_MARKET_WILL_CREATE, NO_CARD_ERRORS, NO_CARD_PROCESSING, NO_CARD_ADD_TO_CAMPAIGN
    pub card_status: Option<OfferCardStatusType>,
    /// Список магазинов, в которых размещен товар.
    /// Статус товара в магазине.
    pub campaigns: Option<Vec<OfferCampaignStatusDTO>>,
    /// Информация о том, какие для товара доступны модели размещения.
    /// Информация о том, по каким моделям можно продавать товар, а по каким нельзя.
    pub selling_programs: Option<Vec<OfferSellingProgramDTO>>,
    /// Товар помещен в архив.
    pub archived: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct OfferWeightDimensionsDTO {
    /// Длина упаковки в см.
    pub length: f64,
    /// Ширина упаковки в см.
    pub width: f64,
    /// Высота упаковки в см.
    pub height: f64,
    /// Вес товара в кг с учетом упаковки (брутто).
    pub weight: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct TimePeriodDTO {
    /// Продолжительность в указанных единицах.
    pub time_period: i32,
    /// Единица измерения.
    /// Enum: HOUR, DAY, WEEK, MONTH, YEAR
    pub time_unit: TimeUnitType,
    /// Комментарий.
    pub comment: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct OfferConditionDTO {
    /// Тип уценки.
    /// Enum: PREOWNED, SHOWCASESAMPLE, REFURBISHED, REDUCTION, RENOVATED
    #[serde(rename = "type")]
    pub condition_type: Option<OfferConditionType>,
    /// Внешний вид товара.
    /// Enum: PERFECT, EXCELLENT, GOOD
    pub quality: Option<OfferConditionQualityType>,
    /// Описание товара. Подробно опишите дефекты, насколько они заметны и где их искать.
    pub reason: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferType {
    /// Лекарства
    Medicine,
    /// Книги
    Book,
    /// Аудиокниги
    Audiobook,
    /// Музыкальная и видеопродукция
    ArtistTitle,
    /// Товары на заказ
    OnDemand,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct AgeDTO {
    pub value: i32,
    pub age_unit: AgeUnitType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct OfferParamDTO {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct GetPriceWithDiscountDTO {
    pub value: Option<f64>,
    pub currency_id: Option<CurrencyId>,
    pub discount_base: Option<f64>,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct GetPriceDTO {
    pub value: f64,
    pub currency_id: CurrencyId,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct OfferCampaignStatusDTO {
    pub campaign_id: i64,
    pub status: OfferCampaignStatusType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct OfferSellingProgramDTO {
    pub selling_program: SellingProgramType,
    pub status: OfferSellingProgramStatusType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnitType {
    Hour,
    Day,
    Week,
    Month,
    Year,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferConditionType {
    Preowned,
    Showcasesample,
    Refurbished,
    Reduction,
    Renovated,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferConditionQualityType {
    Perfect,
    Excellent,
    Good,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgeUnitType {
    Year,
    Month,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferCampaignStatusType {
    Published,
    Checking,
    DisabledByPartner,
    RejectedByMarket,
    DisabledAutomatically,
    CreatingCard,
    NoCard,
    NoStocks,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SellingProgramType {
    Fby,
    Fbs,
    Dbs,
    Express,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferSellingProgramStatusType {
    Fine,
    Reject,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrencyId {
    Rur,
}
fn deserialize_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
        .map_err(serde::de::Error::custom)
}
