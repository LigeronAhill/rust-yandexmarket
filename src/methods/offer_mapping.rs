use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::{debug, instrument};

use crate::{
    models::{
        offer_mappings::{
            AgeDTO, AgeUnitType, CurrencyId, GetOfferMappingDTO, OfferConditionDTO,
            OfferConditionQualityType, OfferConditionType, OfferMappingResponse, OfferParamDTO,
            OfferType, OfferWeightDimensionsDTO, TimePeriodDTO, TimeUnitType,
        },
        ApiResponseStatusType, ErrorResponse,
    },
    MarketClient, OfferMappingRequest,
};
use anyhow::{anyhow, Result};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdatePricesRequest {
    pub offers: Vec<UpdateBusinessOfferPriceDTO>,
}
/// Установка цен
///
/// # Example
///
/// ```rust
/// use rust_yandexmarket::{MarketClient, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let prices = vec![UpdateBusinessOfferPriceDTO::new(
///         "Homakoll_164_Prof_1.3",
///         1074.0,
///         None,
///     )];
///     let _ = client.offer_mappings().update_offers_prices(prices).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateBusinessOfferPriceDTO {
    pub offer_id: String,
    pub price: UpdatePriceWithDiscountDTO,
}
impl UpdateBusinessOfferPriceDTO {
    /// Введите SKU, значение цены и цену до скидки или None
    pub fn new(offer_id: impl Into<String>, value: f64, discount_base: Option<f64>) -> Self {
        Self {
            offer_id: offer_id.into(),
            price: UpdatePriceWithDiscountDTO {
                value,
                currency_id: CurrencyId::Rur,
                discount_base,
            },
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdatePriceWithDiscountDTO {
    pub value: f64,
    pub currency_id: CurrencyId,
    pub discount_base: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateOffersRequest {
    pub offer_mappings: Vec<UpdateOfferMappingDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct DeleteOffersRequest {
    pub offer_ids: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOffersResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<DeleteOffersDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveOffersResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<ArchiveOffersDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveOffersResponse {
    pub status: Option<ApiResponseStatusType>,
    pub result: Option<UnarchiveOffersDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOffersDTO {
    pub not_deleted_offer_ids: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveOffersDTO {
    pub not_archived_offers: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveOffersDTO {
    pub not_unarchived_offer_ids: Option<Vec<String>>,
}
/// Добавление товаров в каталог и редактирование информации о них
///
/// # Example
///
/// ```rust
/// use rust_yandexmarket::{MarketClient, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::fmt::init();
///     let client = MarketClient::init().await?;
///     let update = UpdateOfferMappingDTO::builder()
///         .offer_id("Homakoll_164_Prof_1.3")
///         .name("Клей Homakoll 164 Prof 1.3 кг")
///         .category("Клей")
///         .picture("https://main-cdn.sbermegamarket.ru/big2/hlr-system/335/279/913/730/125/5/600004169210b0.jpeg")
///         .vendor("Homakoll")
///         .description("Для коммерческого (гомогенного и гетерогенного) линолеума. Ковролина. ПВХ плитки. К любым основаниям. Морозостойкий.")
///         .manufacturer_country("Россия")
///         .weight_dimensions(20.0, 20.0, 20.0, 1.3)
///         .build()?;
///     let _ = client.offer_mappings().update_offers(vec![update]).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateOfferMappingDTO {
    pub offer: UpdateOfferDTO,
    pub mapping: Option<UpdateMappingDTO>,
}
impl UpdateOfferMappingDTO {
    pub fn builder() -> UpdateOfferMappingDTOBuilder {
        UpdateOfferMappingDTOBuilder::default()
    }
}
#[derive(Default)]
pub struct UpdateOfferMappingDTOBuilder {
    offer: UpdateOfferDTO,
    mapping: Option<UpdateMappingDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateOfferDTO {
    offer_id: String,
    name: Option<String>,
    category: Option<String>,
    pictures: Option<Vec<String>>,
    videos: Option<Vec<String>>,
    vendor: Option<String>,
    barcodes: Option<Vec<String>>,
    description: Option<String>,
    manufacturer_countries: Option<Vec<String>>,
    weight_dimensions: Option<OfferWeightDimensionsDTO>,
    vendor_code: Option<String>,
    tags: Option<Vec<String>>,
    shelf_life: Option<TimePeriodDTO>,
    life_time: Option<TimePeriodDTO>,
    guarantee_period: Option<TimePeriodDTO>,
    customs_commodity_code: Option<String>,
    certificates: Option<Vec<String>>,
    box_count: Option<i32>,
    condition: Option<OfferConditionDTO>,
    #[serde(rename = "type")]
    offer_type: Option<OfferType>,
    downloadable: Option<bool>,
    adult: Option<bool>,
    age: Option<AgeDTO>,
    params: Option<Vec<OfferParamDTO>>,
    purchase_price: Option<BasePriceDTO>,
    additional_expenses: Option<BasePriceDTO>,
    cofinance_price: Option<BasePriceDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasePriceDTO {
    value: f64,
    currency_id: CurrencyId,
}

impl UpdateOfferMappingDTOBuilder {
    /// Ваш SKU — идентификатор товара в вашей системе.
    /// Разрешена любая последовательность длиной до 80 знаков. В нее могут входить английские и русские буквы, цифры и символы. , / \ () [ ] - = _
    /// Правила использования SKU:
    /// У каждого товара SKU должен быть свой.
    /// SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.
    /// Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.
    pub fn offer_id(&mut self, offer_id: impl Into<String>) -> &mut Self {
        self.offer.offer_id = offer_id.into();
        self
    }
    /// Составляйте название по схеме: тип + бренд или производитель + модель + особенности, если есть (например, цвет, размер или вес) и количество в упаковке.
    /// Не включайте в название условия продажи (например, «скидка», «бесплатная доставка» и т. д.), эмоциональные характеристики («хит», «супер» и т. д.). Не пишите слова большими буквами — кроме устоявшихся названий брендов и моделей.
    /// Оптимальная длина — 50–60 символов, максимальная — 256.
    /// Example: Ударная дрель Makita HP1630, 710 Вт
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.offer.name.insert(name.into());
        self
    }
    /// Категория, к которой магазин относит свой товар. Она помогает точнее определить для товара категорию в каталоге Маркета.
    /// Указывайте конкретные категории — например, набор ножей лучше отнести к категории Столовые приборы, а не просто Посуда.
    /// Выбирайте категории, которые описывают товар, а не абстрактный признак — например, Духи, а не Подарки.
    pub fn category(&mut self, category: impl Into<String>) -> &mut Self {
        let _ = self.offer.category.insert(category.into());
        self
    }
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
    pub fn pictures(&mut self, pictures: Vec<String>) -> &mut Self {
        self.offer.pictures.get_or_insert(vec![]).extend(pictures);
        self
    }
    /// Ссылка на изображения товара.
    /// Указывайте ссылку целиком, включая протокол http или https.
    /// Максимальная длина — 512 символов.
    /// Русские буквы в URL можно.
    /// Можно использовать прямые ссылки на изображения и на Яндекс Диск. Ссылки на Яндекс Диске нужно копировать с помощью функции Поделиться. Относительные ссылки и ссылки на другие облачные хранилища — не работают.
    /// ✅ https://example-shop.ru/images/sku12345.jpg
    /// ✅ https://yadi.sk/i/NaBoRsimVOLov
    /// ❌ /images/sku12345.jpg
    /// ❌ https://www.dropbox.com/s/818f/tovar.jpg
    /// Ссылка на изображение должны быть постоянной. Нельзя использовать динамические ссылки, меняющиеся от выгрузки к выгрузке.
    /// Если нужно заменить изображение, выложите новое изображение по новой ссылке, а ссылку на старое удалите. Если просто заменить изображение по старой ссылке, оно не обновится.
    pub fn picture(&mut self, picture: impl Into<String>) -> &mut Self {
        self.offer
            .pictures
            .get_or_insert(vec![])
            .push(picture.into());
        self
    }
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
    pub fn videos(&mut self, video_url: impl Into<String>) -> &mut Self {
        self.offer
            .videos
            .get_or_insert(vec![])
            .push(video_url.into());
        self
    }
    /// Название бренда или производителя. Должно быть записано так, как его пишет сам бренд.
    /// Example: LEVENHUK
    pub fn vendor(&mut self, vendor: impl Into<String>) -> &mut Self {
        let _ = self.offer.vendor.insert(vendor.into());
        self
    }
    /// Указывайте в виде последовательности цифр. Подойдут коды EAN-13, EAN-8, UPC-A, UPC-E или Code 128.
    /// Для книг указывайте ISBN.
    /// Для товаров определенных категорий и торговых марок штрихкод должен быть действительным кодом GTIN. Обратите внимание: внутренние штрихкоды, начинающиеся на 2 или 02, и коды формата Code 128 не являются GTIN.
    /// Example: 46012300000000
    pub fn barcodes(&mut self, barcodes: Vec<String>) -> &mut Self {
        self.offer.barcodes.get_or_insert(vec![]).extend(barcodes);
        self
    }
    /// Указывайте в виде последовательности цифр. Подойдут коды EAN-13, EAN-8, UPC-A, UPC-E или Code 128.
    /// Для книг указывайте ISBN.
    /// Для товаров определенных категорий и торговых марок штрихкод должен быть действительным кодом GTIN. Обратите внимание: внутренние штрихкоды, начинающиеся на 2 или 02, и коды формата Code 128 не являются GTIN.
    /// Example: 46012300000000
    pub fn barcode(&mut self, barcode: impl Into<String>) -> &mut Self {
        self.offer
            .barcodes
            .get_or_insert(vec![])
            .push(barcode.into());
        self
    }
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
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.offer.description.insert(description.into());
        self
    }
    /// Страна, где был произведен товар.
    /// Записывайте названия стран так, как они записаны в списке.
    /// Example: Россия
    pub fn manufacturer_countries(&mut self, manufacturer_countries: Vec<String>) -> &mut Self {
        self.offer
            .manufacturer_countries
            .get_or_insert(vec![])
            .extend(manufacturer_countries);
        self
    }
    /// Страна, где был произведен товар.
    /// Записывайте названия стран так, как они записаны в списке.
    /// Example: Россия
    pub fn manufacturer_country(&mut self, manufacturer_country: impl Into<String>) -> &mut Self {
        self.offer
            .manufacturer_countries
            .get_or_insert(vec![])
            .push(manufacturer_country.into());
        self
    }
    /// Габариты упаковки и вес товара.
    pub fn weight_dimensions(
        &mut self,
        length: f64,
        width: f64,
        height: f64,
        weight: f64,
    ) -> &mut Self {
        let x = OfferWeightDimensionsDTO {
            length,
            width,
            height,
            weight: Some(weight),
        };
        let _ = self.offer.weight_dimensions.insert(x);
        self
    }
    /// Артикул товара от производителя.
    /// Example: VNDR-0005A
    pub fn vendor_code(&mut self, vendor_code: impl Into<String>) -> &mut Self {
        let _ = self.offer.vendor_code.insert(vendor_code.into());
        self
    }
    /// Метки товара, используемые магазином. Покупателям теги не видны. По тегам можно группировать и фильтровать разные товары в каталоге — например, товары одной серии, коллекции или линейки.
    /// Максимальная длина тега 20 символов. У одного товара может быть максимум 10 тегов. Всего можно создать не больше 50 разных тегов.
    /// Example: до 500 рублей
    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.offer.tags.get_or_insert(vec![]).extend(tags);
        self
    }
    /// Метка товара, используемая магазином. Покупателям теги не видны. По тегам можно группировать и фильтровать разные товары в каталоге — например, товары одной серии, коллекции или линейки.
    /// Максимальная длина тега 20 символов. У одного товара может быть максимум 10 тегов. Всего можно создать не больше 50 разных тегов.
    /// Example: до 500 рублей
    pub fn tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.offer.tags.get_or_insert(vec![]).push(tag.into());
        self
    }
    /// Срок годности — период, по прошествии которого товар становится непригоден.
    /// Указывайте срок, указанный на банке или упаковке. Текущая дата, дата поставки или дата отгрузки значения не имеет.
    /// Обязательно указывайте срок, если он есть.
    /// В комментарии укажите условия хранения. Например, «Хранить в сухом помещении».
    pub fn shelf_life(
        &mut self,
        time_period: i32,
        time_unit: TimeUnitType,
        comment: Option<String>,
    ) -> &mut Self {
        let x = TimePeriodDTO {
            time_period,
            time_unit,
            comment,
        };
        let _ = self.offer.shelf_life.insert(x);
        self
    }
    /// Срок службы — период, в течение которого товар должен исправно выполнять свою функцию.
    /// Обязательно указывайте срок, если он есть.
    /// В комментарии укажите условия хранения. Например, «Использовать при температуре не ниже −10 градусов».
    pub fn life_time(
        &mut self,
        time_period: i32,
        time_unit: TimeUnitType,
        comment: Option<String>,
    ) -> &mut Self {
        let x = TimePeriodDTO {
            time_period,
            time_unit,
            comment,
        };
        let _ = self.offer.life_time.insert(x);
        self
    }
    /// Гарантийный срок — период, в течение которого можно бесплатно заменить или починить товар.
    /// Обязательно указывайте срок, если он есть.
    /// В комментарии опишите особенности гарантийного обслуживания. Например, «Гарантия на аккумулятор — 6 месяцев».
    pub fn guarantee_period(
        &mut self,
        time_period: i32,
        time_unit: TimeUnitType,
        comment: Option<String>,
    ) -> &mut Self {
        let x = TimePeriodDTO {
            time_period,
            time_unit,
            comment,
        };
        let _ = self.offer.guarantee_period.insert(x);
        self
    }
    /// Код товара в единой Товарной номенклатуре внешнеэкономической деятельности (ТН ВЭД) — 10 или 14 цифр без пробелов.
    /// Обязательно укажите, если он есть.
    /// Example: 8517610008
    pub fn customs_commodity_code(
        &mut self,
        customs_commodity_code: impl Into<String>,
    ) -> &mut Self {
        let _ = self
            .offer
            .customs_commodity_code
            .insert(customs_commodity_code.into());
        self
    }
    /// Номера документов на товар: сертификата, декларации соответствия и т. п.
    /// Передавать можно только номера документов, сканы которого загружены в личном кабинете продавца по инструкции.
    pub fn certificates(&mut self, certificates: Vec<String>) -> &mut Self {
        self.offer
            .certificates
            .get_or_insert(vec![])
            .extend(certificates);
        self
    }
    /// Номера документов на товар: сертификата, декларации соответствия и т. п.
    /// Передавать можно только номера документов, сканы которого загружены в личном кабинете продавца по инструкции.
    pub fn certificate(&mut self, certificate: impl Into<String>) -> &mut Self {
        self.offer
            .certificates
            .get_or_insert(vec![])
            .push(certificate.into());
        self
    }
    /// Количество грузовых мест.
    /// Параметр используется, если товар представляет собой несколько коробок, упаковок и так далее. Например, кондиционер занимает два места — внешний и внутренний блоки в двух коробках.
    /// Для товаров, занимающих одно место, не передавайте этот параметр.
    pub fn box_count(&mut self, box_count: i32) -> &mut Self {
        let _ = self.offer.box_count.insert(box_count);
        self
    }
    /// Состояние уцененного товара.
    /// Используется только для товаров, продаваемых с уценкой.
    pub fn condition(
        &mut self,
        condition_type: Option<OfferConditionType>,
        quality: Option<OfferConditionQualityType>,
        reason: Option<String>,
    ) -> &mut Self {
        let x = OfferConditionDTO {
            condition_type,
            quality,
            reason,
        };
        let _ = self.offer.condition.insert(x);
        self
    }
    /// Особый тип товара. Указывается, если товар — книга, аудиокнига, лекарство, музыка, видео или поставляется под заказ.
    /// Enum: DEFAULT, MEDICINE, BOOK, AUDIOBOOK, ARTIST_TITLE, ON_DEMAND
    pub fn offer_type(&mut self, offer_type: OfferType) -> &mut Self {
        let _ = self.offer.offer_type.insert(offer_type);
        self
    }
    /// Признак цифрового товара. Укажите true, если товар доставляется по электронной почте.
    pub fn downloadable(&mut self, downloadable: bool) -> &mut Self {
        let _ = self.offer.downloadable.insert(downloadable);
        self
    }
    /// Параметр включает для товара пометку 18+. Устанавливайте ее только для товаров, которые относятся к удовлетворению сексуальных потребностей.
    pub fn adult(&mut self, adult: bool) -> &mut Self {
        let _ = self.offer.adult.insert(adult);
        self
    }
    /// Если товар не предназначен для детей младше определенного возраста, укажите это.
    /// Возрастное ограничение можно задавать в годах (с нуля, с 6, 12, 16 или 18) или в месяцах (любое число от 0 до 12).
    pub fn age(&mut self, value: i32, age_unit: AgeUnitType) -> &mut Self {
        let x = AgeDTO { value, age_unit };
        let _ = self.offer.age.insert(x);
        self
    }
    /// Характеристики, которые есть только у товаров конкретной категории — например, диаметр колес велосипеда или материал подошвы обуви.
    /// Параметры товара.
    /// Используйте POST businesses/{businessId}/offer-cards/update для передачи характеристик товара, которые специфичны для его категории. Так переданные характеристики с большей вероятностью попадут на карточку.
    pub fn params(&mut self, params: Vec<OfferParamDTO>) -> &mut Self {
        self.offer.params.get_or_insert(vec![]).extend(params);
        self
    }
    /// Характеристики, которые есть только у товаров конкретной категории — например, диаметр колес велосипеда или материал подошвы обуви.
    /// Параметры товара.
    /// Используйте POST businesses/{businessId}/offer-cards/update для передачи характеристик товара, которые специфичны для его категории. Так переданные характеристики с большей вероятностью попадут на карточку.
    pub fn param(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let x = OfferParamDTO {
            name: name.into(),
            value: value.into(),
        };
        self.offer.params.get_or_insert(vec![]).push(x);
        self
    }
    /// Себестоимость — затраты на самостоятельное производство товара или закупку у производителя или поставщиков.
    /// Цена на товар.
    /// Время последнего обновления.
    pub fn purchase_price(&mut self, value: f64) -> &mut Self {
        let x = BasePriceDTO {
            value,
            currency_id: CurrencyId::Rur,
        };
        let _ = self.offer.purchase_price.insert(x);
        self
    }
    /// Дополнительные расходы на товар. Например, на доставку или упаковку.
    /// Цена на товар.
    /// Время последнего обновления.
    pub fn additional_expenses(&mut self, value: f64) -> &mut Self {
        let x = BasePriceDTO {
            value,
            currency_id: CurrencyId::Rur,
        };
        let _ = self.offer.additional_expenses.insert(x);
        self
    }
    /// Цена для скидок с Маркетом
    /// Маркет может компенсировать до половины скидки. Назначьте минимальную цену до вычета тарифов, по которой готовы продавать товар, а мы рассчитаем скидку и размер софинансирования.
    /// Если Маркет не готов софинансировать скидку, покупатель её не увидит.
    /// Цена на товар.
    /// Время последнего обновления.
    pub fn cofinance_price(&mut self, value: f64) -> &mut Self {
        let x = BasePriceDTO {
            value,
            currency_id: CurrencyId::Rur,
        };
        let _ = self.offer.cofinance_price.insert(x);
        self
    }
    pub fn mapping(&mut self, market_sku: i64) -> &mut Self {
        let _ = self.mapping.insert(UpdateMappingDTO { market_sku });
        self
    }
    pub fn build(&self) -> Result<UpdateOfferMappingDTO> {
        if self.offer.offer_id.is_empty() {
            Err(anyhow!("offer_id required!"))
        } else {
            Ok(UpdateOfferMappingDTO {
                offer: self.offer.to_owned(),
                mapping: self.mapping.to_owned(),
            })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateMappingDTO {
    pub market_sku: i64,
}

impl MarketClient {
    /// Каталог
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let offers = client.offer_mappings().get_all_offers().await?;
    ///     dbg!(offers);
    ///     let update = UpdateOfferMappingDTO::builder()
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .name("Клей Homakoll 164 Prof 1.3 кг")
    ///         .category("Клей")
    ///         .picture("https://main-cdn.sbermegamarket.ru/big2/hlr-system/335/279/913/730/125/5/600004169210b0.jpeg")
    ///         .vendor("Homakoll")
    ///         .description("Для коммерческого (гомогенного и гетерогенного) линолеума. Ковролина. ПВХ плитки. К любым основаниям. Морозостойкий.")
    ///         .manufacturer_country("Россия")
    ///         .weight_dimensions(20.0, 20.0, 20.0, 1.3)
    ///         .build()?;
    ///     let _ = client.offer_mappings().update_offers(vec![update]).await?;
    ///     tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    ///     let prices = vec![UpdateBusinessOfferPriceDTO::new(
    ///         "Homakoll_164_Prof_1.3",
    ///         1074.0,
    ///         None,
    ///     )];
    ///     let _ = client.offer_mappings().update_offers_prices(prices).await?;
    ///     let not_archived = client.offer_mappings().archive_offers(vec!["Homakoll_164_Prof_1.3".to_string()]).await?;
    ///     dbg!(not_archived);
    ///     tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    ///     let not_unarchived = client.offer_mappings().unarchive_offers(vec!["Homakoll_164_Prof_1.3".to_string()]).await?;
    ///     dbg!(not_unarchived);
    ///     let to_delete = vec!["Homakoll_164_Prof_1.3".to_string()];
    ///     let not_deleted = client.offer_mappings().delete_offers(to_delete).await?;
    ///     dbg!(not_deleted);
    ///     Ok(())
    /// }
    /// ```
    pub fn offer_mappings(&self) -> OfferMapping {
        OfferMapping { api_client: self }
    }
}
#[derive(Debug, Clone)]
pub struct OfferMapping<'a> {
    api_client: &'a MarketClient,
}
impl<'a> OfferMapping<'a> {
    /// Возвращает список товаров в каталоге с параметрами каждого товара.
    ///
    /// Можно использовать тремя способами:
    ///
    /// Задать список интересующих SKU;
    /// задать фильтр — в этом случае результаты возвращаются постранично;
    /// не передавать тело запроса, чтобы получить список всех товаров в каталоге.
    /// ⚙️ Лимит: 600 запросов в минуту, не более 200 товаров в одном запросе
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::MarketClient;
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::from_env().await?;
    ///     let offers = client.offer_mappings().get_all_offers().await?;
    ///     println!("{}", offers.len());
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get_all_offers(&self) -> Result<Vec<GetOfferMappingDTO>> {
        debug!("Getting offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut next_page_token = None;
        let mut result = Vec::new();
        loop {
            let response = match next_page_token {
                Some(page_token) => {
                    self.api_client
                        .client()
                        .post(&uri)
                        .query(&[("page_token", page_token)])
                        .bearer_auth(self.api_client.access_token())
                        .send()
                        .await?
                }
                None => {
                    self.api_client
                        .client()
                        .post(&uri)
                        .bearer_auth(self.api_client.access_token())
                        .send()
                        .await?
                }
            };
            match response.status() {
                StatusCode::OK => {
                    let offers_response: OfferMappingResponse = response.json().await?;
                    result.extend(offers_response.result.offer_mappings);
                    next_page_token = offers_response.result.paging.next_page_token;
                    if next_page_token.is_none() {
                        break;
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while getting offer-mappings\n{error:#?}");
                    return Err(anyhow!(msg))
                }
            }
        }
        Ok(result)
    }
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, OfferMappingRequest};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let req = OfferMappingRequest::builder()
    ///         .vendor_name("Homakoll")
    ///         .build();
    ///     let filtered_offers = client
    ///         .offer_mappings()
    ///         .get_all_offers_with_filter(req)
    ///         .await?;
    ///     dbg!(filtered_offers);
    /// Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get_all_offers_with_filter(
        &self,
        req: OfferMappingRequest,
    ) -> Result<Vec<GetOfferMappingDTO>> {
        debug!("Getting offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut next_page_token = None;
        let mut result = Vec::new();
        loop {
            let response = match next_page_token {
                Some(page_token) => {
                    self.api_client
                        .client()
                        .post(&uri)
                        .query(&[("page_token", page_token)])
                        .bearer_auth(self.api_client.access_token())
                        .json(&req)
                        .send()
                        .await?
                }
                None => {
                    self.api_client
                        .client()
                        .post(&uri)
                        .bearer_auth(self.api_client.access_token())
                        .json(&req)
                        .send()
                        .await?
                }
            };
            match response.status() {
                StatusCode::OK => {
                    let offers_response: OfferMappingResponse = response.json().await?;
                    result.extend(offers_response.result.offer_mappings);
                    next_page_token = offers_response.result.paging.next_page_token;
                    if next_page_token.is_none() {
                        break;
                    }
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while getting offer-mappings\n{error:#?}");
                    return Err(anyhow!(msg));
                }
            }
        }
        Ok(result)
    }
    /// Добавляет товары в каталог или редактирует информацию об уже имеющихся товарах.
    ///
    /// Чтобы добавить новый товар, передайте его с новым идентификатором, который раньше никогда не использовался в каталоге. Старайтесь сразу передать как можно больше информации — она потребуется Маркету для подбора подходящей карточки или создания новой. Если известно, какой карточке на Маркете соответствует товар, можно сразу указать идентификатор этой карточки (SKU на Маркете) в поле marketSKU.
    ///
    /// Для новых товаров обязательно укажите параметры: offerId, name, category, pictures, vendor, description.
    ///
    /// Чтобы отредактировать информацию о товаре, передайте новые данные, указав в offerId соответствующий ваш SKU. Поля, в которых ничего не меняется, можно не передавать.
    ///
    /// Правила использования SKU
    ///
    /// У каждого товара SKU должен быть свой.
    ///
    /// SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.
    ///
    /// Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.
    ///
    /// Данные в каталоге обновляются не мгновенно
    ///
    /// Это занимает до нескольких минут.
    ///
    /// ⚙️ Лимит: 5000 товаров в минуту, не более 500 товаров в одном запросе
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, UpdateOfferMappingDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let update = UpdateOfferMappingDTO::builder()
    ///         .offer_id("Homakoll_164_Prof_1.3")
    ///         .name("Клей Homakoll 164 Prof 1.3 кг")
    ///         .category("Клей")
    ///         .picture("https://main-cdn.sbermegamarket.ru/big2/hlr-system/335/279/913/730/125/5/600004169210b0.jpeg")
    ///         .vendor("Homakoll")
    ///         .description("Для коммерческого (гомогенного и гетерогенного) линолеума. Ковролина. ПВХ плитки. К любым основаниям. Морозостойкий.")
    ///         .manufacturer_country("Россия")
    ///         .weight_dimensions(20.0, 20.0, 20.0, 1.3)
    ///         .build()?;
    ///     let _ = client.offer_mappings().update_offers(vec![update]).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn update_offers(&self, offer_mappings: Vec<UpdateOfferMappingDTO>) -> Result<()> {
        debug!("Updating offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings/update",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut errors = String::new();
        for offers in offer_mappings.chunks(500) {
            let update = UpdateOffersRequest {
                offer_mappings: offers.to_vec(),
            };
            let response = self
                .api_client
                .client()
                .post(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&update)
                .send()
                .await?;
            debug!("sleeping 6 secs");
            tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
            match response.status() {
                StatusCode::OK => continue,
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while updating offer-mappings\n{error:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow!(errors))
        }
    }
    /// Удаляет товары из каталога.
    ///
    /// ⚙️ Лимит: 5000 товаров в минуту, не более 200 товаров в одном запросе
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, UpdateOfferMappingDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let to_delete = vec!["Homakoll_164_Prof_1.3".to_string()];
    ///     let not_deleted = client.offer_mappings().delete_offers(to_delete).await?;
    ///     dbg!(not_deleted);
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn delete_offers(&self, offer_ids: Vec<String>) -> Result<Vec<String>> {
        debug!("Deleting offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings/delete",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut errors = String::new();
        let mut result = Vec::new();
        for offers in offer_ids.chunks(200) {
            let req = DeleteOffersRequest {
                offer_ids: offers.to_vec(),
            };

            let response = self
                .api_client
                .client()
                .post(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&req)
                .send()
                .await?;
            debug!("sleeping 2.4 secs");
            tokio::time::sleep(tokio::time::Duration::from_millis(2400)).await;
            match response.status() {
                StatusCode::OK => {
                    let res: DeleteOffersResponse = response.json().await?;
                    let Some(dto) = res.result else {
                        continue;
                    };
                    let Some(not_deleted) = dto.not_deleted_offer_ids else {
                        continue;
                    };
                    result.extend(not_deleted);
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while updating offer-mappings\n{error:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(result)
        } else {
            Err(anyhow!(errors))
        }
    }
    /// Установка цен
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let prices = vec![UpdateBusinessOfferPriceDTO::new(
    ///         "Homakoll_164_Prof_1.3",
    ///         1074.0,
    ///         None,
    ///     )];
    ///     let _ = client.offer_mappings().update_offers_prices(prices).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn update_offers_prices(
        &self,
        offers: Vec<UpdateBusinessOfferPriceDTO>,
    ) -> Result<()> {
        debug!("Updating offers prices");
        let uri = format!(
            "{}businesses/{}/offer-prices/updates",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut errors = String::new();
        for offers_part in offers.chunks(500) {
            let req = UpdatePricesRequest {
                offers: offers_part.to_vec(),
            };
            let response = self
                .api_client
                .client()
                .post(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&req)
                .send()
                .await?;
            debug!("sleeping 6 secs");
            tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
            match response.status() {
                StatusCode::OK => continue,
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while updating offers prices\n{error:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow!(errors))
        }
    }
    /// Помещает товары в архив. Товары, помещенные в архив, скрыты с витрины во всех магазинах кабинета.
    ///
    /// В архив нельзя отправить товар, который хранится на складе Маркета
    ///
    /// Вначале такой товар нужно распродать или вывезти.
    ///
    /// ⚙️ Лимит: 5000 товаров в минуту, не более 200 товаров в одном запросе
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let not_archived = client.offer_mappings().archive_offers(vec!["Homakoll_164_Prof_1.3".to_string()]).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn archive_offers(&self, offer_ids: Vec<String>) -> Result<Vec<String>> {
        debug!("Archiving offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings/archive",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut errors = String::new();
        let mut result = Vec::new();
        for offers in offer_ids.chunks(200) {
            let req = DeleteOffersRequest {
                offer_ids: offers.to_vec(),
            };

            let response = self
                .api_client
                .client()
                .post(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&req)
                .send()
                .await?;
            debug!("sleeping 2.4 secs");
            tokio::time::sleep(tokio::time::Duration::from_millis(2400)).await;
            match response.status() {
                StatusCode::OK => {
                    let res: ArchiveOffersResponse = response.json().await?;
                    let Some(dto) = res.result else {
                        continue;
                    };
                    let Some(not_archived) = dto.not_archived_offers else {
                        continue;
                    };
                    result.extend(not_archived);
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while updating offer-mappings\n{error:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(result)
        } else {
            Err(anyhow!(errors))
        }
    }
    /// Восстанавливает товары из архива.
    ///
    /// ⚙️ Лимит: 5000 товаров в минуту, не более 200 товаров в одном запросе
    ///
    /// ```rust
    /// use rust_yandexmarket::{MarketClient, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let client = MarketClient::init().await?;
    ///     let not_unarchived = client.offer_mappings().unarchive_offers(vec!["Homakoll_164_Prof_1.3".to_string()]).await?;
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn unarchive_offers(&self, offer_ids: Vec<String>) -> Result<Vec<String>> {
        debug!("Unarchiving offers");
        let uri = format!(
            "{}businesses/{}/offer-mappings/unarchive",
            crate::BASE_URL,
            self.api_client.business_id()
        );
        let mut errors = String::new();
        let mut result = Vec::new();
        for offers in offer_ids.chunks(200) {
            let req = DeleteOffersRequest {
                offer_ids: offers.to_vec(),
            };

            let response = self
                .api_client
                .client()
                .post(&uri)
                .bearer_auth(self.api_client.access_token())
                .json(&req)
                .send()
                .await?;
            debug!("sleeping 2.4 secs");
            tokio::time::sleep(tokio::time::Duration::from_millis(2400)).await;
            match response.status() {
                StatusCode::OK => {
                    let res: UnarchiveOffersResponse = response.json().await?;
                    let Some(dto) = res.result else {
                        continue;
                    };
                    let Some(not_unarchived) = dto.not_unarchived_offer_ids else {
                        continue;
                    };
                    result.extend(not_unarchived);
                }
                _ => {
                    let error: ErrorResponse = response.json().await?;
                    let msg = format!("Error while updating offer-mappings\n{error:#?}\n");
                    errors.push_str(&msg);
                }
            }
        }
        if errors.is_empty() {
            Ok(result)
        } else {
            Err(anyhow!(errors))
        }
    }
}
