//! `Authentication` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::authentication::*;

/// Handle for the `Authentication` resource — auth mode + password.
#[derive(Debug)]
pub struct AuthenticationService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> AuthenticationService<'c> {
    /// `GET /v2.2/auth` — current auth mode + allowlist.
    pub async fn get(&self) -> Result<AuthGetData, ApiError> {
        self.client
            .transport
            .request::<(), AuthGetData>(Method::GET, "/v2.2/auth", &[], None, true)
            .await
    }

    /// `PUT /v2.2/auth` — change auth mode and/or password.
    pub async fn update(&self, body: &AuthPutRequest) -> Result<AuthPutData, ApiError> {
        self.client
            .transport
            .request(Method::PUT, "/v2.2/auth", &[], Some(body), true)
            .await
    }
}
