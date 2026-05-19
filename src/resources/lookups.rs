//! `Lookups` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::lookups::*;

/// Handle for the `Lookups` resource — CNAM and LRN dips.
///
/// Each call costs money; rate them per call rather than fanning out blindly.
#[derive(Debug)]
pub struct LookupsService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> LookupsService<'c> {
    /// `GET /v2.2/cnam/{number}` — CNAM dip on `number` (10-digit TN).
    pub async fn cnam(&self, number: &str) -> Result<CnamData, ApiError> {
        let path = format!("/v2.2/cnam/{}", number);
        self.client
            .transport
            .request::<(), CnamData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `GET /v2.2/lrn/{number}/{ani}` — LRN dip. `ani` is the presented ANI
    /// (10-digit TN) used only for billing/auth — it is not echoed back.
    pub async fn lrn(&self, number: &str, ani: &str) -> Result<LrnLookupData, ApiError> {
        let path = format!("/v2.2/lrn/{}/{}", number, ani);
        self.client
            .transport
            .request::<(), LrnLookupData>(Method::GET, &path, &[], None, true)
            .await
    }
}
