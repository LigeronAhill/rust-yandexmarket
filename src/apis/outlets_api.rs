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

/// struct for typed errors of method [`create_outlet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOutletError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_outlet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOutletError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_outlet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOutletError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_outlets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOutletsError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_outlet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOutletError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Создает точку продаж магазина на Маркете.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn create_outlet(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    change_outlet_request: crate::models::ChangeOutletRequest,
) -> Result<crate::models::CreateOutletResponse, Error<CreateOutletError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets",
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
    local_var_req_builder = local_var_req_builder.json(&change_outlet_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateOutletError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Удаляет точку продаж магазина на Маркете.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn delete_outlet(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    outlet_id: i64,
) -> Result<crate::models::EmptyApiResponse, Error<DeleteOutletError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets/{outletId}",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        outletId = outlet_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteOutletError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Возвращает информацию о точках продаж магазина.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn get_outlet(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    outlet_id: i64,
) -> Result<crate::models::GetOutletResponse, Error<GetOutletError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets/{outletId}",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        outletId = outlet_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetOutletError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Возвращает список точек продаж магазина.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn get_outlets(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    page_token: Option<&str>,
    region_id: Option<i64>,
    shop_outlet_code: Option<&str>,
    region_id2: Option<i64>,
) -> Result<crate::models::GetOutletsResponse, Error<GetOutletsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets",
        local_var_configuration.base_path,
        campaignId = campaign_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("region_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = shop_outlet_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("shop_outlet_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region_id2 {
        local_var_req_builder =
            local_var_req_builder.query(&[("regionId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetOutletsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Изменяет информацию о точке продаж магазина на Маркете.  В течение суток этим и другими запросами о точках продаж, кроме запроса [GET delivery/services](../../reference/orders/getDeliveryServices.md), можно получить и изменить информацию об определенном суммарном количестве точек продаж. Оно зависит от количества точек продаж магазина.  |**⚙️ Лимит:** 100 000 запросов в час| |-|
pub async fn update_outlet(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    outlet_id: i64,
    change_outlet_request: crate::models::ChangeOutletRequest,
) -> Result<crate::models::EmptyApiResponse, Error<UpdateOutletError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/outlets/{outletId}",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        outletId = outlet_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&change_outlet_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateOutletError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
