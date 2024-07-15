/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GenerateReportDto : Идентификатор, который понадобится для отслеживания статуса генерации и получения готового отчета.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateReportDto {
    /// Идентификатор, который понадобится для отслеживания статуса генерации и получения готового отчета.
    #[serde(rename = "reportId")]
    pub report_id: String,
    /// Ожидаемая продолжительность генерации в миллисекундах.
    #[serde(rename = "estimatedGenerationTime")]
    pub estimated_generation_time: i64,
}

impl GenerateReportDto {
    /// Идентификатор, который понадобится для отслеживания статуса генерации и получения готового отчета.
    pub fn new(report_id: String, estimated_generation_time: i64) -> GenerateReportDto {
        GenerateReportDto {
            report_id,
            estimated_generation_time,
        }
    }
}