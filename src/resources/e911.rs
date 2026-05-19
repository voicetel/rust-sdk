//! `e911` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::e911::*;

/// Handle for the `e911` resource.
#[derive(Debug)]
pub struct E911Service<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> E911Service<'c> {
    /// `GET /v2.2/e911` — every e911 record on the account.
    pub async fn list(&self) -> Result<E911AllData, ApiError> {
        self.client
            .transport
            .request::<(), E911AllData>(Method::GET, "/v2.2/e911", &[], None, true)
            .await
    }

    /// `POST /v2.2/e911` — validate + provision in one call.
    pub async fn create(&self, body: &E911CreateRequest) -> Result<E911RecordData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/e911", &[], Some(body), true)
            .await
    }

    /// `POST /v2.2/e911/validations` — validate an address; returns an `addressid`.
    pub async fn validate(&self, body: &E911AddressRequest) -> Result<E911ValidateData, ApiError> {
        self.client
            .transport
            .request(
                Method::POST,
                "/v2.2/e911/validations",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// `GET /v2.2/e911/{dn}` — fetch the e911 record for `dn`.
    pub async fn get(&self, dn: &str) -> Result<E911RecordData, ApiError> {
        let path = format!("/v2.2/e911/{}", dn);
        self.client
            .transport
            .request::<(), E911RecordData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/e911/{dn}` — provision using a previously-validated `addressid`.
    pub async fn provision(
        &self,
        dn: &str,
        body: &E911ProvisionByIdRequest,
    ) -> Result<E911RecordData, ApiError> {
        let path = format!("/v2.2/e911/{}", dn);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/e911/{dn}` — remove the e911 record. Returns `()` on 204.
    pub async fn remove(&self, dn: &str) -> Result<(), ApiError> {
        let path = format!("/v2.2/e911/{}", dn);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }
}
