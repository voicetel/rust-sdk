//! `Gateways` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::gateways::*;

/// Handle for the `Gateways` resource.
#[derive(Debug)]
pub struct GatewaysService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> GatewaysService<'c> {
    /// `GET /v2.2/gateways` — every gateway on the account.
    pub async fn list(&self) -> Result<GatewaysListData, ApiError> {
        self.client
            .transport
            .request::<(), GatewaysListData>(Method::GET, "/v2.2/gateways", &[], None, true)
            .await
    }

    /// `POST /v2.2/gateways` — create a new gateway.
    pub async fn add(&self, body: &GatewayAddRequest) -> Result<GatewayEntry, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/gateways", &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/gateways/{id}` — fetch a single gateway.
    pub async fn get(&self, id: i64) -> Result<GatewayEntry, ApiError> {
        let path = format!("/v2.2/gateways/{}", id);
        self.client
            .transport
            .request::<(), GatewayEntry>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/gateways/{id}` — partial-update a gateway.
    pub async fn update(
        &self,
        id: i64,
        body: &GatewayUpdateRequest,
    ) -> Result<GatewayEntry, ApiError> {
        let path = format!("/v2.2/gateways/{}", id);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/gateways/{id}` — remove a gateway. Returns `()` on 204.
    pub async fn remove(&self, id: i64) -> Result<(), ApiError> {
        let path = format!("/v2.2/gateways/{}", id);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// `GET /v2.2/gateways/{id}/numbers` — TNs routed through this gateway.
    pub async fn numbers(&self, id: i64) -> Result<GatewayNumbersData, ApiError> {
        let path = format!("/v2.2/gateways/{}/numbers", id);
        self.client
            .transport
            .request::<(), GatewayNumbersData>(Method::GET, &path, &[], None, true)
            .await
    }
}
