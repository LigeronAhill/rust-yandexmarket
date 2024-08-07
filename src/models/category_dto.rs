/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};

/// CategoryDto : Информация о категории.  Категория считается листовой, если у нее нет дочерних категорий.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryDto {
    /// Идентификатор категории.
    pub id: i64,
    /// Название категории.
    pub name: String,
    /// Дочерние категории.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<CategoryDto>>,
}

impl CategoryDto {
    /// Информация о категории.  Категория считается листовой, если у нее нет дочерних категорий.
    pub fn new(id: i64, name: String) -> CategoryDto {
        CategoryDto {
            id,
            name,
            children: None,
        }
    }
}
