/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::models::{BasePriceDto, CurrencyType, OfferWeightDimensionsDto, UpdatePriceWithDiscountDto};

/// UpdateOfferDto : Параметры товара.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Builder)]
pub struct UpdateOfferDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId")]
    #[builder(setter(into))]
    pub offer_id: String,
    /// Составляйте название по схеме: тип + бренд или производитель + модель + особенности, если есть (например, цвет, размер или вес) и количество в упаковке.  Не включайте в название условия продажи (например, «скидка», «бесплатная доставка» и т. д.), эмоциональные характеристики («хит», «супер» и т. д.). Не пишите слова большими буквами — кроме устоявшихся названий брендов и моделей.  Оптимальная длина — 50–60 символов, максимальная — 256.  [Рекомендации и правила](https://yandex.ru/support/marketplace/assortment/fields/title.html)
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub name: Option<String>,
    /// Идентификатор категории на Маркете, к которой вы относите свой товар.  Если не указать `marketCategoryId`, то маркетная категория будет определена автоматически.  При изменении информации о товаре передавайте тот же идентификатор категории. Если вы укажете другой, категория товара не поменяется. Изменить ее можно только в кабинете продавца на Маркете.  Список категорий Маркета можно получить с помощью запроса  [POST categories/tree](../../reference/categories/getCategoriesTree.md).
    #[serde(rename = "marketCategoryId", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub market_category_id: Option<i64>,
    /// Категория товара в вашем магазине. Значение будет использовано для определения категории товара на Маркете в случае, если вы не передали категорию в параметре `marketCategoryId`.  Указывайте конкретные категории — например, набор ножей лучше отнести к категории **Столовые приборы**, а не просто **Посуда**.  Выбирайте категории, которые описывают товар, а не абстрактный признак — например, **Духи**, а не **Подарки**.  Значение будет использовано для определения категории товара на Маркете в случае, если вы не передали категорию в параметре `marketCategoryId`.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub category: Option<String>,
    /// Ссылки на изображения товара. Изображение по первой ссылке считается основным, остальные дополнительными.  **Требования к ссылкам**  * Ссылок может быть до 30. * Указывайте ссылку целиком, включая протокол http или https. * Максимальная длина — 512 символов. * Русские буквы в URL можно. * Можно использовать прямые ссылки на изображения и на Яндекс Диск. Ссылки на Яндекс Диске нужно копировать с помощью функции **Поделиться**. Относительные ссылки и ссылки на другие облачные хранилища — не работают.  ✅ `https://example-shop.ru/images/sku12345.jpg`  ✅ `https://yadi.sk/i/NaBoRsimVOLov`  ❌ `/images/sku12345.jpg`  ❌ `https://www.dropbox.com/s/818f/tovar.jpg`  Ссылки на изображение должны быть постоянными. Нельзя использовать динамические ссылки, меняющиеся от выгрузки к выгрузке.  Если нужно заменить изображение, выложите новое изображение по новой ссылке, а ссылку на старое удалите. Если просто заменить изображение по старой ссылке, оно не обновится.  [Требования к изображениям](https://yandex.ru/support/marketplace/assortment/fields/images.html)
    #[serde(rename = "pictures", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub pictures: Option<Vec<String>>,
    /// Ссылка (URL) на видео товара.  Максимальное количество ссылок — 6.  **Требования к ссылке**  * Указывайте ссылку целиком, включая протокол http или https. * Максимальная длина — 512 символов. * Русские буквы в URL можно. * Можно использовать прямые ссылки на видео и на Яндекс Диск. Ссылки на Яндекс Диске нужно копировать с помощью функции **Поделиться**. Относительные ссылки и ссылки на другие облачные хранилища — не работают.  ✅ `https://example-shop.ru/video/sku12345.avi`  ✅ `https://yadi.sk/i/NaBoRsimVOLov`  ❌ `/video/sku12345.avi`  ❌ `https://www.dropbox.com/s/818f/super-tovar.avi`  Ссылки на видео должны быть постоянными. Нельзя использовать динамические ссылки, меняющиеся от выгрузки к выгрузке.  Если нужно заменить видео, выложите новое видео по новой ссылке, а ссылку на старое удалите. Если просто заменить видео по старой ссылке, оно не обновится.  [Требования к видео](https://yandex.ru/support/marketplace/assortment/fields/video.html)
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub videos: Option<Vec<String>>,
    /// Список инструкций по использованию товара.  Максимальное количество инструкций — 6.  Если вы передадите пустое поле `manuals`, загруженные ранее инструкции удалятся.
    #[serde(rename = "manuals", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub manuals: Option<Vec<crate::models::OfferManualDto>>,
    /// Название бренда или производителя. Должно быть записано так, как его пишет сам бренд.
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub vendor: Option<String>,
    /// Указывайте в виде последовательности цифр. Подойдут коды EAN-13, EAN-8, UPC-A, UPC-E или Code 128.  Для книг указывайте ISBN.  Для товаров [определенных категорий и торговых марок](https://yastatic.net/s3/doc-binary/src/support/market/ru/yandex-market-list-for-gtin.xlsx) штрихкод должен быть действительным кодом GTIN. Обратите внимание: внутренние штрихкоды, начинающиеся на 2 или 02, и коды формата Code 128 не являются GTIN.  [Что такое GTIN](*gtin)  
    #[serde(rename = "barcodes", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub barcodes: Option<Vec<String>>,
    /// Подробное описание товара: например, его преимущества и особенности.  Не давайте в описании инструкций по установке и сборке. Не используйте слова «скидка», «распродажа», «дешевый», «подарок» (кроме подарочных категорий), «бесплатно», «акция», «специальная цена», «новинка», «new», «аналог», «заказ», «хит». Не указывайте никакой контактной информации и не давайте ссылок.  Можно использовать теги:  * \\<h>, \\<h1>, \\<h2> и так далее — для заголовков; * \\<br> и \\<p> — для переноса строки; * \\<ol> — для нумерованного списка; * \\<ul> — для маркированного списка; * \\<li> — для создания элементов списка (должен находиться внутри \\<ol> или \\<ul>); * \\<div> — поддерживается, но не влияет на отображение текста.  Оптимальная длина — 400–600 символов, максимальная — 6000.  [Рекомендации и правила](https://yandex.ru/support/marketplace/assortment/fields/description.html)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub description: Option<String>,
    /// Страна, где был произведен товар.  Записывайте названия стран так, как они записаны в [списке](https://yastatic.net/s3/doc-binary/src/support/market/ru/countries.xlsx).
    #[serde(
        rename = "manufacturerCountries",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(setter(into, strip_option))]
    pub manufacturer_countries: Option<Vec<String>>,
    /// Габариты упаковки и вес товара.
    /// В см и кг
    #[serde(rename = "weightDimensions", skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom))]
    pub weight_dimensions: Option<Box<OfferWeightDimensionsDto>>,
    /// Артикул товара от производителя.
    #[serde(rename = "vendorCode", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub vendor_code: Option<String>,
    /// Метки товара, используемые магазином. Покупателям теги не видны. По тегам можно группировать и фильтровать разные товары в каталоге — например, товары одной серии, коллекции или линейки.  Максимальная длина тега 20 символов. У одного товара может быть максимум 10 тегов. Всего можно создать не больше 50 разных тегов.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "shelfLife", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shelf_life: Option<Box<crate::models::TimePeriodDto>>,
    #[serde(rename = "lifeTime", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub life_time: Option<Box<crate::models::TimePeriodDto>>,
    #[serde(rename = "guaranteePeriod", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub guarantee_period: Option<Box<crate::models::TimePeriodDto>>,
    /// Код товара в единой Товарной номенклатуре внешнеэкономической деятельности (ТН ВЭД) — 10 или 14 цифр без пробелов.  Обязательно укажите, если он есть.
    #[serde(
        rename = "customsCommodityCode",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub customs_commodity_code: Option<String>,
    /// Номера документов на товар: сертификата, декларации соответствия и т. п.  Передавать можно только номера документов, сканы которого загружены в кабинете продавца по [инструкции](https://yandex.ru/support/marketplace/assortment/restrictions/certificates.html).
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub certificates: Option<Vec<String>>,
    /// Количество грузовых мест.  Параметр используется, если товар представляет собой несколько коробок, упаковок и так далее. Например, кондиционер занимает два места — внешний и внутренний блоки в двух коробках.  Для товаров, занимающих одно место, не передавайте этот параметр.
    #[serde(rename = "boxCount", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    #[builder(default)]
    pub box_count: Option<i32>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    #[builder(default)]
    pub condition: Option<Box<crate::models::OfferConditionDto>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    #[builder(default)]
    pub r#type: Option<crate::models::OfferType>,
    /// Признак цифрового товара. Укажите `true`, если товар доставляется по электронной почте.  [Как работать с цифровыми товарами](../../step-by-step/digital.md)
    #[serde(rename = "downloadable", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    #[builder(default)]
    pub downloadable: Option<bool>,
    /// Параметр включает для товара пометку 18+. Устанавливайте ее только для товаров, которые относятся к удовлетворению сексуальных потребностей.
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    #[builder(default)]
    pub adult: Option<bool>,
    #[serde(rename = "age", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    #[builder(default)]
    pub age: Option<Box<crate::models::AgeDto>>,
    /// Список характеристик с их значениями.  С `parameterValues` обязательно передавайте `marketCategoryId` — идентификатор категории на Маркете, к которой относятся указанные характеристики товара.  Всегда обновляется целиком.  Максимальное количество характеристик — 300.
    #[serde(rename = "parameterValues", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub parameter_values: Option<Vec<crate::models::ParameterValueDto>>,
    #[serde(rename = "basicPrice", skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom))]
    pub basic_price: Option<Box<UpdatePriceWithDiscountDto>>,
    #[serde(rename = "purchasePrice", skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom))]
    pub purchase_price: Option<Box<BasePriceDto>>,
    #[serde(rename = "additionalExpenses", skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom))]
    pub additional_expenses: Option<Box<BasePriceDto>>,
    #[serde(rename = "cofinancePrice", skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom))]
    pub cofinance_price: Option<Box<BasePriceDto>>,
}

impl UpdateOfferDto {
    pub fn builder() -> UpdateOfferDtoBuilder {
        UpdateOfferDtoBuilder::default()
    }
}
impl UpdateOfferDtoBuilder {
    /// Габариты упаковки и вес товара.
    /// В см и кг
    pub fn weight_dimensions(&mut self, length: f32, width: f32, height: f32, weight: f32) -> &mut Self {
        let dimensions = OfferWeightDimensionsDto::new(length, width, height, weight);
        self.weight_dimensions = Some(Some(Box::new(dimensions)));
        self
    }
    pub fn basic_price(&mut self, value: f32, discount_base: Option<f32>) -> &mut Self {
        let bp = UpdatePriceWithDiscountDto::new(value, discount_base, CurrencyType::Rur);
        self.basic_price = Some(Some(Box::new(bp)));
        self
    }
    pub fn purchase_price(&mut self, value: f32) -> &mut Self {
        let pp = BasePriceDto::new(value, CurrencyType::Rur);
        self.purchase_price = Some(Some(Box::new(pp)));
        self
    }
    pub fn additional_expenses(&mut self, value: f32) -> &mut Self {
        let pp = BasePriceDto::new(value, CurrencyType::Rur);
        self.additional_expenses = Some(Some(Box::new(pp)));
        self
    }
    pub fn cofinance_price(&mut self, value: f32) -> &mut Self {
        let pp = BasePriceDto::new(value, CurrencyType::Rur);
        self.cofinance_price = Some(Some(Box::new(pp)));
        self
    }
}
