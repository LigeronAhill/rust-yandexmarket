# rust-yandexmarket

## Библиотека для работы с API Yandex.Market на языке программирования Rust

## Использование


 Клиент для доступа к Yandex Market API.

 # Пример

 ```rust
use anyhow::Result;
use rust_yandexmarket::MarketClient;

 #[tokio::main]
 async fn main() -> Result<()> {
     let subscriber = tracing_subscriber::fmt()
         .with_max_level(tracing::Level::DEBUG)
         .finish();
     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
     let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
     let client = MarketClient::new(token).await?;
     // do something with the client
     Ok(())
 }
```

## Как добавить новые товары в каталог

1. Получите список категорий Маркета, выполнив запрос [`get_categories_tree`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/categories/getCategoriesTree).
2. Для каждой категории запросите список необходимых характеристик с помощью [`get_category_content_parameters`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/content/getCategoryContentParameters).
3. Передайте информацию о товарах (названия, описания, фотографии и так далее), цены, категории на Маркете и характеристики с помощью запроса [`update_offer_mappings`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/updateOfferMappings).
4. Чтобы узнать стоимость услуг Маркета для конкретных товаров, передайте их параметры в запросе [`tariffs_calculate`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/tariffs/calculateTariffs).
5. Получите у Маркета список моделей, по которым можно продавать каждый из добавленных товаров с помощью запроса [`offer_mappings`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/getOfferMappings). [Что такое модель работы и какие модели есть](https://yandex.ru/support/marketplace/introduction/models.html).
6. Задайте условия размещения товаров с помощью запроса [`offers_update`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/assortment/updateCampaignOffers). Условия размещения — это минимальный объем заказа, квант продаж и ставка НДС. Если вы работаете по модели DBS, этим же запросом задаются параметры доставки.
7. Убедитесь, что товары появились на витрине, с помощью запроса [`get_campaign_offers`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/assortment/getCampaignOffers).
Подробные пояснения к статусам товаров вы найдете в [Справке Маркета для продавцов](https://yandex.ru/support/marketplace/assortment/add/statuses.html).

## Как изменить цены на товары

1. Чтобы узнать стоимость услуг Маркета для конкретных товаров, передайте их параметры в запросе [`tariffs_calculate`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/tariffs/calculateTariffs).
2. Передайте новые цены для всех магазинов с помощью запроса [`offer_prices_updates`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/updateBusinessPrices).
3. Убедитесь, что ни один из товаров не попал в карантин с помощью запроса [`price_quarantine`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/getBusinessQuarantineOffers).
4. Если карантин не пустой, проверьте цены на товары. Ошибочно установленные цены для всех магазинов можно исправить запросом [`offer_prices_updates`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/updateBusinessPrices).
5. После того как в карантине останутся только правильные цены, подтвердите их запросом [`price_quarantine_confirm`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/confirmBusinessPrices). Если ложные срабатывания карантина случаются часто, подумайте о том, чтобы изменить его порог по [инструкции](https://yandex.ru/support/marketplace/assortment/operations/prices.html#quarantine).

## Как управлять товарами в архиве
1. Для архивации товаров используйте запрос [`offer_mappings_archive`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/addOffersToArchive). Если товары не удалось архивировать, они вернутся в ответе запроса.
2. Для просмотра товаров в архиве используйте фильтр `archived` в запросе [`offer_mappings`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/getOfferMappings)
3. Чтобы восстановить товар из архива, используйте запрос [`offer_mappings_unarchive`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/business-assortment/deleteOffersFromArchive)

## Передача остатков по API
С помощью запроса [`update_stock`](https://yandex.ru/dev/market/partner-api/doc/ru/reference/stocks/updateStocks)
