/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`delete_outlet_licenses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOutletLicensesError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_outlet_licenses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOutletLicensesError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_outlet_licenses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOutletLicensesError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Удаляет информацию о лицензиях для точек продаж.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn delete_outlet_licenses(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    ids: Option<Vec<i64>>,
) -> Result<crate::models::EmptyApiResponse, Error<DeleteOutletLicensesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets/licenses",
        local_var_configuration.base_path,
        campaignId = campaign_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("ids".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "ids",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteOutletLicensesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Возвращает информацию о лицензиях для точек продаж.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn get_outlet_licenses(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    outlet_ids: Option<Vec<i64>>,
    ids: Option<Vec<i64>>,
) -> Result<crate::models::GetOutletLicensesResponse, Error<GetOutletLicensesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets/licenses",
        local_var_configuration.base_path,
        campaignId = campaign_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = outlet_ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("outletIds".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "outletIds",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("ids".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "ids",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetOutletLicensesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Передает информацию о новых и существующих лицензиях для точек продаж. Поддерживаются только лицензии на розничную продажу алкоголя.  Чтобы размещать алкогольную продукцию на Маркете, надо также прислать гарантийное письмо (если вы еще не делали этого раньше) и правильно оформить предложения в прайс-листе. Далее информация о лицензиях проходит проверку.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn update_outlet_licenses(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    update_outlet_license_request: crate::models::UpdateOutletLicenseRequest,
) -> Result<crate::models::EmptyApiResponse, Error<UpdateOutletLicensesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets/licenses",
        local_var_configuration.base_path,
        campaignId = campaign_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_outlet_license_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateOutletLicensesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}