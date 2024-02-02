use rust_yandexmarket::{MarketClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    let outlets = client.outlets().get_all_outlets().await?;
    let id = outlets.first().unwrap().id;
    let _outlet = client.outlets().get_outlet(id).await?;
    let address = rust_yandexmarket::Address::builder()
        .region_id(13)
        .street("улица Ленина")
        .number("69")
        .block("5")
        .additional("Вход со двора")
        .build();
    let schedule_item_1 = rust_yandexmarket::WorkingScheduleItem::builder()
        .start_day(rust_yandexmarket::DayOfWeekType::Monday)
        .end_day(rust_yandexmarket::DayOfWeekType::Friday)
        .start_time("09:00")
        .end_time("21:00")
        .build();
    let schedule_item_2 = rust_yandexmarket::WorkingScheduleItem::builder()
        .start_day(rust_yandexmarket::DayOfWeekType::Saturday)
        .end_day(rust_yandexmarket::DayOfWeekType::Sunday)
        .start_time("10:00")
        .end_time("18:00")
        .build();
    let delivery_rule = rust_yandexmarket::DeliveryRule::builder()
        .cost(0)
        .min_delivery_days(5)
        .max_delivery_days(7)
        .order_before(15)
        .build()?;
    let outlet_to_create = rust_yandexmarket::Outlet::builder()
        .name("Test Outlet")
        .outlet_type(rust_yandexmarket::OutletType::Retail)
        .coords("20.45, 54.71")
        .is_main(false)
        .shop_outlet_code("42")
        .visibility(rust_yandexmarket::OutletVisibilityType::Hidden)
        .address(address)
        .phone("+7 (999) 696-69-69")
        .phone("+7 (888) 999-66-99")
        .phones(vec![
            "+7 (678) 321-65-49".to_string(),
            "+7 (987) 654-32-11".to_string(),
        ])
        .work_in_holiday(true)
        .schedule_item(schedule_item_1)
        .schedule_item(schedule_item_2)
        .delivery_rule(delivery_rule)
        .email("most@wanted.man")
        .storage_period(3)
        .build();
    let created = client.outlets().create_outlet(outlet_to_create).await?;
    let created_outlet = client.outlets().get_outlet(created).await?;
    println!("{created_outlet:#?}");
    let mut outlet_to_update = created_outlet;
    outlet_to_update.name = "Another name".to_string();
    let _ = client
        .outlets()
        .update_outlet(created, outlet_to_update)
        .await?;
    let _deleted = client.outlets().delete_outlet(created).await?;
    Ok(())
}
