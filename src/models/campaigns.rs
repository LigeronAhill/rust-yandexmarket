use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResponse {
    /// Список с информацией по каждому магазину.
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
/// Модель для пагинации.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlippingPagerDTO {
    /// Сколько всего найдено элементов.
    pub total: i32,
    /// Начальный номер найденного элемента на странице.
    pub from: i32,
    /// Конечный номер найденного элемента на странице.
    pub to: i32,
    /// Текущая страница.
    pub current_page: i32,
    /// Общее количество страниц.
    pub pages_count: i32,
    /// Размер страницы.
    pub page_size: i32,
}
/// Информацию о кабинете.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDTO {
    /// Идентификатор кабинета.
    pub id: i64,
    /// Название бизнеса.
    pub name: String,
}
/// Модель, по которой работает магазин:
///
/// - `FBS` — FBS или Экспресс;
/// - `FBY` — FBY;
/// - `DBS` — DBS.
/// Enum: FBS, FBY, DBS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlacementType {
    FBS,
    FBY,
    DBS,
}
