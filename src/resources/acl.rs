//! `ACL` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::acl::*;

/// Handle for the `ACL` resource — manages the IP allowlist on the account.
#[derive(Debug)]
pub struct AclService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> AclService<'c> {
    /// `GET /v2.2/acl` — current allowlist.
    pub async fn list(&self) -> Result<AclListData, ApiError> {
        self.client
            .transport
            .request::<(), AclListData>(Method::GET, "/v2.2/acl", &[], None, true)
            .await
    }

    /// `POST /v2.2/acl` — add CIDR entries.
    pub async fn add(&self, body: &AclModifyRequest) -> Result<AclAddData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/acl", &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/acl` — remove CIDR entries. Returns 200 with a body (not 204).
    pub async fn remove(&self, body: &AclModifyRequest) -> Result<AclRemoveData, ApiError> {
        self.client
            .transport
            .request(Method::DELETE, "/v2.2/acl", &[], Some(body), true)
            .await
    }
}
