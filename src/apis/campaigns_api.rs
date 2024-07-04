use super::{configuration, Error};
use crate::apis::ResponseContent;
use crate::models::{CampaignDto, GetCampaignsResponse};
use crate::MarketClient;
use anyhow::{anyhow, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CampaignError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}
//
// /// struct for typed errors of method [`get_campaign`]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum GetCampaignError {
//     Status400(crate::models::ApiClientDataErrorResponse),
//     Status401(crate::models::ApiUnauthorizedErrorResponse),
//     Status403(crate::models::ApiForbiddenErrorResponse),
//     Status404(crate::models::ApiNotFoundErrorResponse),
//     Status420(crate::models::ApiLimitErrorResponse),
//     Status500(crate::models::ApiServerErrorResponse),
//     UnknownValue(serde_json::Value),
// }
//
// /// struct for typed errors of method [`get_campaign_logins`]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum GetCampaignLoginsError {
//     Status400(crate::models::ApiClientDataErrorResponse),
//     Status401(crate::models::ApiUnauthorizedErrorResponse),
//     Status403(crate::models::ApiForbiddenErrorResponse),
//     Status404(crate::models::ApiNotFoundErrorResponse),
//     Status420(crate::models::ApiLimitErrorResponse),
//     Status500(crate::models::ApiServerErrorResponse),
//     UnknownValue(serde_json::Value),
// }
//
// /// struct for typed errors of method [`get_campaign_region`]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum GetCampaignRegionError {
//     Status400(crate::models::ApiClientDataErrorResponse),
//     Status401(crate::models::ApiUnauthorizedErrorResponse),
//     Status403(crate::models::ApiForbiddenErrorResponse),
//     Status404(crate::models::ApiNotFoundErrorResponse),
//     Status420(crate::models::ApiLimitErrorResponse),
//     Status500(crate::models::ApiServerErrorResponse),
//     UnknownValue(serde_json::Value),
// }
//
// /// struct for typed errors of method [`get_campaign_settings`]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum GetCampaignSettingsError {
//     Status400(crate::models::ApiClientDataErrorResponse),
//     Status401(crate::models::ApiUnauthorizedErrorResponse),
//     Status403(crate::models::ApiForbiddenErrorResponse),
//     Status404(crate::models::ApiNotFoundErrorResponse),
//     Status420(crate::models::ApiLimitErrorResponse),
//     Status500(crate::models::ApiServerErrorResponse),
//     UnknownValue(serde_json::Value),
// }
//
// /// struct for typed errors of method [`get_campaigns`]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum GetCampaignsError {
//     Status400(crate::models::ApiClientDataErrorResponse),
//     Status401(crate::models::ApiUnauthorizedErrorResponse),
//     Status403(crate::models::ApiForbiddenErrorResponse),
//     Status404(crate::models::ApiNotFoundErrorResponse),
//     Status420(crate::models::ApiLimitErrorResponse),
//     Status500(crate::models::ApiServerErrorResponse),
//     UnknownValue(serde_json::Value),
// }
//
// /// struct for typed errors of method [`get_campaigns_by_login`]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum GetCampaignsByLoginError {
//     Status400(crate::models::ApiClientDataErrorResponse),
//     Status401(crate::models::ApiUnauthorizedErrorResponse),
//     Status403(crate::models::ApiForbiddenErrorResponse),
//     Status404(crate::models::ApiNotFoundErrorResponse),
//     Status420(crate::models::ApiLimitErrorResponse),
//     Status500(crate::models::ApiServerErrorResponse),
//     UnknownValue(serde_json::Value),
// }

impl MarketClient {
    /// Возвращает информацию о магазине. |**⚙️ Лимит:** 1000 запросов в час| |-|
    pub async fn get_campaign(
        configuration: &configuration::Configuration,
        campaign_id: i64,
    ) -> Result<crate::models::GetCampaignResponse> {
        let local_var_configuration = configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/campaigns/{campaignId}",
            local_var_configuration.base_path,
            campaignId = campaign_id
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let res = serde_json::from_str(&local_var_content)?;
            Ok(res)
        } else {
            let local_var_entity: Option<CampaignError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(anyhow!(Error::ResponseError(local_var_error)))
        }
    }

    /// Возвращает список логинов, у которых есть доступ к магазину. |**⚙️ Лимит:** 1000 запросов в час| |-|
    pub async fn get_campaign_logins(
        configuration: &configuration::Configuration,
        campaign_id: i64,
    ) -> Result<crate::models::GetCampaignLoginsResponse> {
        let local_var_configuration = configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/campaigns/{campaignId}/logins",
            local_var_configuration.base_path,
            campaignId = campaign_id
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let res = serde_json::from_str(&local_var_content)?;
            Ok(res)
        } else {
            let local_var_entity: Option<CampaignError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(anyhow!(Error::ResponseError(local_var_error)))
        }
    }

    /// Возвращает регион, в котором находится магазин. |**⚙️ Лимит:** 5 000 запросов в час| |-|
    pub async fn get_campaign_region(
        configuration: &configuration::Configuration,
        campaign_id: i64,
    ) -> Result<crate::models::GetCampaignRegionResponse> {
        let local_var_configuration = configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/campaigns/{campaignId}/region",
            local_var_configuration.base_path,
            campaignId = campaign_id
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let res = serde_json::from_str(&local_var_content)?;
            Ok(res)
        } else {
            let local_var_entity: Option<CampaignError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(anyhow!(Error::ResponseError(local_var_error)))
        }
    }

    /// Возвращает информацию о настройках магазина, идентификатор которого указан в запросе. |**⚙️ Лимит:** 1000 запросов в час| |-|
    pub async fn get_campaign_settings(
        configuration: &configuration::Configuration,
        campaign_id: i64,
    ) -> Result<crate::models::GetCampaignSettingsResponse> {
        let local_var_configuration = configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/campaigns/{campaignId}/settings",
            local_var_configuration.base_path,
            campaignId = campaign_id
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let res = serde_json::from_str(&local_var_content)?;
            Ok(res)
        } else {
            let local_var_entity: Option<CampaignError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(anyhow!(Error::ResponseError(local_var_error)))
        }
    }

    /// Возвращает список магазинов, к которым имеет доступ пользователь — владелец авторизационного токена, использованного в запросе. Для агентских пользователей список состоит из подагентских магазинов. |**⚙️ Лимит:** 1000 запросов в час| |-|
    pub async fn get_campaigns(&self) -> Result<Vec<CampaignDto>> {
        let mut url = self.base_path().join("campaigns")?;
        url.set_query(Some("pageSize=100"));
        let mut page = 1;
        let mut result = Vec::new();
        loop {
            let query = format!("page={page}");
            let mut local_url = Url::from_str(url.as_str())?;
            url.clone_into(&mut local_url);
            local_url.set_query(Some(&query));
            let local_var_resp = self
                .client()
                .get(local_url)
                .bearer_auth(self.access_token())
                .send()
                .await?;
            let local_var_status = local_var_resp.status();
            let local_var_content = local_var_resp.text().await?;
            if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
                let local_res: GetCampaignsResponse =
                    serde_json::from_str(&local_var_content)?;
                result.extend(local_res.campaigns);
                if local_res.pager.is_some_and(|pager| {
                    pager.pages_count.is_some_and(|total| {
                        pager.current_page.is_some_and(|current| current < total)
                    })
                }) {
                    page += 1;
                } else {
                    break;
                }
            } else {
                let local_var_entity: Option<CampaignError> =
                    serde_json::from_str(&local_var_content).ok();
                let local_var_error = ResponseContent {
                    status: local_var_status,
                    content: local_var_content,
                    entity: local_var_entity,
                };
                return Err(anyhow!("{}", Error::ResponseError(local_var_error)));
            }
        }
        Ok(result.into_iter().flatten().collect())
    }

    /// Возвращает список магазинов, к которым у пользователя с указанным логином есть доступ. |**⚙️ Лимит:** 100 запросов в час| |-|
    pub async fn get_campaigns_by_login(
        configuration: &configuration::Configuration,
        login: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<GetCampaignsResponse> {
        let local_var_configuration = configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/campaigns/by_login/{login}",
            local_var_configuration.base_path,
            login = crate::apis::urlencode(login)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = page {
            local_var_req_builder =
                local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page_size {
            local_var_req_builder =
                local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let res = serde_json::from_str(&local_var_content)?;
            Ok(res)
        } else {
            let local_var_entity: Option<CampaignError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(anyhow!(Error::ResponseError(local_var_error)))
        }
    }
}
