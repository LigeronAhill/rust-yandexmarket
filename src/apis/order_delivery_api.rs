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

/// struct for typed errors of method [`get_order_buyer_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderBuyerInfoError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_order_delivery_date`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetOrderDeliveryDateError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_order_delivery_track_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetOrderDeliveryTrackCodeError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_order_storage_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrderStorageLimitError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_order_eac`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyOrderEacError {
    Status400(crate::models::ApiClientDataErrorResponse),
    Status401(crate::models::ApiUnauthorizedErrorResponse),
    Status403(crate::models::ApiForbiddenErrorResponse),
    Status404(crate::models::ApiNotFoundErrorResponse),
    Status420(crate::models::ApiLimitErrorResponse),
    Status500(crate::models::ApiServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Возвращает информацию о покупателе по идентификатору заказа.  {% note info \"\" %}  Чтобы получить информацию о покупателе, который является юридическим лицом, воспользуйтесь запросом [POST campaigns/{campaignId}/orders/{orderId}/business-buyer](../../reference/order-business-information/getOrderBusinessBuyerInfo.md).  {% endnote %}  Получить данные можно, только если заказ находится в статусе `PROCESSING`, `DELIVERY` или `PICKUP`.  Если у вас настроена работа с push-компонентом, Маркет отправит уведомление о смене статуса заказа с помощью запроса [POST order/status](../../pushapi/reference/postOrderStatus.md).  |**⚙️ Лимит:** 3 000 запросов в час| |-|
pub async fn get_order_buyer_info(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    order_id: i64,
) -> Result<crate::models::GetOrderBuyerInfoResponse, Error<GetOrderBuyerInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/orders/{orderId}/buyer",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        orderId = order_id
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
        let local_var_entity: Option<GetOrderBuyerInfoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Метод изменяет дату доставки заказа в статусе `PROCESSING` или `DELIVERY`. Для заказов с другими статусами дату доставки изменить нельзя.  |**⚙️ Лимит:** 1 000 000 запросов в час| |-|
pub async fn set_order_delivery_date(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    order_id: i64,
    set_order_delivery_date_request: crate::models::SetOrderDeliveryDateRequest,
) -> Result<crate::models::EmptyApiResponse, Error<SetOrderDeliveryDateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/orders/{orderId}/delivery/date",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        orderId = order_id
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
    local_var_req_builder = local_var_req_builder.json(&set_order_delivery_date_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetOrderDeliveryDateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// {% note warning \"Этот запрос только для DBS\" %}  Вы можете использовать запрос, чтобы повысить лояльность покупателей. Если они могут узнать, на каком этапе доставки находятся их заказы, доверие покупателей к вашему магазину может возрасти.  {% endnote %}  Передает Маркету трек‑номер, по которому покупатель может отследить посылку со своим заказом через службу доставки.  Передать трек‑номер можно, только если заказ находится в статусе `PROCESSING`, `DELIVERY` или `PICKUP`.  |**⚙️ Лимит:** 1 000 000 запросов в час| |-|
pub async fn set_order_delivery_track_code(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    order_id: i64,
    set_order_delivery_track_code_request: crate::models::SetOrderDeliveryTrackCodeRequest,
) -> Result<crate::models::EmptyApiResponse, Error<SetOrderDeliveryTrackCodeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/orders/{orderId}/delivery/track",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        orderId = order_id
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
    local_var_req_builder = local_var_req_builder.json(&set_order_delivery_track_code_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetOrderDeliveryTrackCodeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Продлевает срок хранения заказа в пункте выдачи продавца.  Заказ должен быть в статусе `PICKUP`. Продлить срок можно только один раз, не больше чем на 30 дней.  Новый срок хранения можно получить в параметре `outletStorageLimitDate` запроса [GET campaigns/{campaignId}/orders/{orderId}](../../reference/orders/getOrder.md).  |**⚙️ Лимит:** 1 000 000 запросов в час| |-|
pub async fn update_order_storage_limit(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    order_id: i64,
    update_order_storage_limit_request: crate::models::UpdateOrderStorageLimitRequest,
) -> Result<crate::models::EmptyApiResponse, Error<UpdateOrderStorageLimitError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/orders/{orderId}/delivery/storage-limit",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        orderId = order_id
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
    local_var_req_builder = local_var_req_builder.json(&update_order_storage_limit_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateOrderStorageLimitError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Отправляет Маркету код подтверждения для его проверки.  Код подтверждает передачу заказа или невыкупа:  * курьеру — курьер должен назвать магазину код; * магазину — магазин называет код курьеру.  Если магазин получает невыкупленный заказ, то ему нужно назвать курьеру код из кабинета или приложения.  Если у магазина настроена работа с кодами подтверждения, в запросах [POST order/status](../../pushapi/reference/postOrderStatus.md), [PUT campaigns/{campaignId}/orders/{orderId}/status](../../reference/orders/updateOrderStatus.md), [GET campaigns/{campaignId}/orders](../../reference/orders/getOrders.md), [GET campaigns/{campaignId}/orders/{orderId}](../../reference/orders/getOrder.md) в параметре `delivery`, вложенном в `order` будет возвращаться параметр `eacType` с типом `Enum` — тип кода подтверждения для передачи заказа.  Возможные значения: `MERCHANT_TO_COURIER` — магазин называет код курьеру, `COURIER_TO_MERCHANT` — курьер называет код магазину.  Параметр `eacType` возвращается при статусах заказа `COURIER_FOUND`, `COURIER_ARRIVED_TO_SENDER` и `DELIVERY_SERVICE_UNDELIVERED`. Если заказ в других статусах, параметр может отсутствовать.  |**⚙️ Лимит:** 1 000 000 запросов в час| |-|
pub async fn verify_order_eac(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    order_id: i64,
    verify_order_eac_request: crate::models::VerifyOrderEacRequest,
) -> Result<crate::models::VerifyOrderEacResponse, Error<VerifyOrderEacError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/campaigns/{campaignId}/orders/{orderId}/verifyEac",
        local_var_configuration.base_path,
        campaignId = campaign_id,
        orderId = order_id
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
    local_var_req_builder = local_var_req_builder.json(&verify_order_eac_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VerifyOrderEacError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}