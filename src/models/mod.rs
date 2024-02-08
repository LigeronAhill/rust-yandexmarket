use serde::{Deserialize, Serialize};

pub mod campaigns;
pub mod geobases;
pub mod offer_cards;
pub mod offer_mappings;
pub mod outlets;
pub mod sales_managment;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApiResponseStatusType {
    Ok,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDTO {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: Option<ApiResponseStatusType>,
    pub error: Option<Vec<ApiErrorDTO>>,
}
/// Модель для пагинации.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlippingPagerDTO {
    /// Сколько всего найдено элементов.
    pub total: Option<i32>,
    /// Начальный номер найденного элемента на странице.
    pub from: i32,
    /// Конечный номер найденного элемента на странице.
    pub to: i32,
    /// Текущая страница.
    pub current_page: i32,
    /// Общее количество страниц.
    pub pages_count: Option<i32>,
    /// Размер страницы.
    pub page_size: i32,
}
/// Информация о страницах результатов.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrollingPagerDTO {
    /// Идентификатор следующей страницы результатов.
    pub next_page_token: Option<String>,
    /// Идентификатор предыдущей страницы результатов.
    pub prev_page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardScrollingPagerDTO {
    /// Идентификатор следующей страницы результатов.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}
