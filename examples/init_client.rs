use anyhow::Result;
use rust_yandexmarket::MarketClient;
use rust_yandexmarket::models::CategoryDto;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    let all_categories = client.get_categories().await?;
    let carpet_category = all_categories
        .into_iter()
        .flat_map(|c| search_in_categories_by_name(c, "ковролин"))
        .collect::<Vec<_>>();
    let carpet_category_parameters = client.get_category_parameters(carpet_category[0].id).await?;
    dbg!(&carpet_category_parameters);
    Ok(())
}
fn search_in_categories_by_name(category: CategoryDto, search_string: &str) -> Option<CategoryDto> {
    if category.name.to_lowercase().contains(&search_string.to_lowercase()) {
        return Some(category)
    } else {
        if let Some(children) = category.children {
            for category in children {
                if let Some(category) = search_in_categories_by_name(category, search_string) {
                    return Some(category)
                }
            }
        }
    }
    None
}