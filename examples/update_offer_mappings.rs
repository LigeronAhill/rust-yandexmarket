use anyhow::Result;
use rust_yandexmarket::models::{
    ParameterValueDto, UpdateOfferDto, UpdateOfferMappingDto,
};
use rust_yandexmarket::MarketClient;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    info!("Client initialized successfully\n{client:#?}");
    let business_id = 919862;
    let category_id = 6119048;
    let content_parameters = client
        .get_category_content_parameters(category_id)
        .await?
        .result
        .and_then(|r| r.parameters)
        .unwrap_or_default();
    let mut parameters = Vec::new();
    for content_parameter in content_parameters.iter() {
        if let Some(name) = content_parameter.name.as_deref() {
            let parameter_id = content_parameter.id;
            let mut unit_id = None;
            let mut value_id = None;
            let value = match name {
                "Ширина" => {
                    unit_id = content_parameter.get_unit_id("метр");
                    "0.8"
                }
                "Форма" => {
                    value_id = content_parameter.get_value_id("прямоугольная");
                    "прямоугольная"
                }
                "Цвет товара для карточки" => {
                    value_id = content_parameter.get_value_id("75");
                    "75"
                }
                "Количество в наборе" => "2",
                "Длина" => "1.5",
                "Цвет товара для фильтра" => {
                    value_id = content_parameter.get_value_id("серый");
                    "серый"
                }
                "Вес" => "6.72",
                "Толщина" => {
                    unit_id = content_parameter.get_unit_id("миллиметр");
                    "15.5"
                }
                "Материал основы" => {
                    value_id = content_parameter.get_value_id("джут");
                    "джут"
                }
                "Материал верха" => {
                    value_id = content_parameter.get_value_id("полиамид");
                    "полиамид"
                }
                "Тип" => {
                    value_id = content_parameter.get_value_id("ковер");
                    "ковер"
                }
                "Тип рисунка" => {
                    value_id = content_parameter.get_value_id("однотонный");
                    "однотонный"
                }
                "Способ производства" => {
                    value_id = content_parameter.get_value_id("машинный");
                    "машинный"
                }
                "Противоскользящая основа" => "false",
                "Безворсовый" => "false",
                "Вес ворса на квадратный метр" => {
                    unit_id = content_parameter.get_unit_id("г/м²");
                    "2100"
                }
                "Высота ворса" => {
                    unit_id = content_parameter.get_unit_id("миллиметр");
                    "13"
                }
                "Вес на квадратный метр" => {
                    unit_id = content_parameter.get_unit_id("г/м²");
                    "2800"
                }
                "Страна производства" => {
                    value_id = content_parameter.get_value_id("Бельгия");
                    "Бельгия"
                }
                "Набор" => "true",
                _ => continue,
            };
            let pvd = ParameterValueDto::build()
                .parameter_id(parameter_id)
                .unit_id(unit_id)
                .value_id(value_id)
                .value(value)
                .build()?;
            parameters.push(pvd);
        }
    }
    let pictures = vec![
        "https://safira.club/wp-content/uploads/mekota_75_large.jpeg".to_string(),
        "https://safira.club/wp-content/uploads/mekota_75_office_large.jpeg".to_string(),
    ];
    let offer = UpdateOfferDto::builder()
        .offer_id("AW Carolus 75 0.8x1.5 - 2 pcs")
        .name("Ковер AW Carolus 75 0.8x1.5 м комплект 2 штуки")
        .market_category_id(category_id)
        .category("Ковры")
        .pictures(pictures)
        .vendor("AW")
        .description("Ковёр AW Carolus – это качественный и эстетичный элемент декора из Бельгии. Это однотонный ковёр из 100% полиэстера с окантованными неширокой тесьмой краями.")
        .manufacturer_countries(vec!["Бельгия".to_string()])
        .weight_dimensions(80.0, 40.0, 40.0, 6.72)
        .vendor_code("AW Carolus 75")
        .parameter_values(parameters)
        .basic_price(27490.0, Some(35350.0))
        .purchase_price(15340.0)
        .additional_expenses(2900.0)
        .cofinance_price(21990.0)
        .build()?;
    let update_offers_mapping_dto = vec![UpdateOfferMappingDto::new(offer)];
    let update_result = client
        .update_offer_mappings(business_id, update_offers_mapping_dto)
        .await?;
    info!("Update result:\n{:#?}", update_result);
    Ok(())
}
