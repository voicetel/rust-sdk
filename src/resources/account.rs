//! `Account` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::account::*;

/// Handle for the `Account` resource.
///
/// **Note:** `cdr`, `recurring_charges`, `payments`, `registration`, and
/// `info` (plus [`Client::login`](crate::Client::login)) share a 6 req/hour/IP
/// rate limit on the server. Bursting will trigger 429s.
#[derive(Debug)]
pub struct AccountService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> AccountService<'c> {
    /// `GET /v2.2/account` — the authenticated account's profile.
    pub async fn get(&self) -> Result<AccountData, ApiError> {
        self.client
            .transport
            .request::<(), AccountData>(Method::GET, "/v2.2/account", &[], None, true)
            .await
    }

    /// `PUT /v2.2/account` — partial update of account settings.
    pub async fn update(&self, body: &AccountPutRequest) -> Result<AccountPutData, ApiError> {
        self.client
            .transport
            .request(Method::PUT, "/v2.2/account", &[], Some(body), true)
            .await
    }

    /// `POST /v2.2/account` — admin-only sub-account creation.
    pub async fn add(&self, body: &AccountAddRequest) -> Result<AccountAddData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/account", &[], Some(body), true)
            .await
    }

    /// `POST /v2.2/accounts` — public sign-up flow.
    pub async fn signup(&self, body: &AccountSignupRequest) -> Result<AccountSignupData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/accounts", &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/account/cdr` — call detail records in `[start, end]` (unix-seconds).
    /// Rate-limited (6/hr/IP).
    pub async fn cdr(&self, start: i64, end: i64) -> Result<AccountCdrData, ApiError> {
        let mut q: Vec<(&str, String)> = Vec::new();
        if start != 0 {
            q.push(("start", start.to_string()));
        }
        if end != 0 {
            q.push(("end", end.to_string()));
        }
        self.client
            .transport
            .request::<(), AccountCdrData>(Method::GET, "/v2.2/account/cdr", &q, None, true)
            .await
    }

    /// `GET /v2.2/account/credits` — credit history (newest first).
    pub async fn credits(&self) -> Result<AccountCreditsData, ApiError> {
        self.client
            .transport
            .request::<(), AccountCreditsData>(
                Method::GET,
                "/v2.2/account/credits",
                &[],
                None,
                true,
            )
            .await
    }

    /// `GET /v2.2/account/recurring-charges` — active recurring charges. Rate-limited.
    pub async fn recurring_charges(&self) -> Result<AccountMrcData, ApiError> {
        self.client
            .transport
            .request::<(), AccountMrcData>(
                Method::GET,
                "/v2.2/account/recurring-charges",
                &[],
                None,
                true,
            )
            .await
    }

    /// `GET /v2.2/account/payments` — payment history (newest first). Rate-limited.
    pub async fn payments(&self) -> Result<AccountPaymentsData, ApiError> {
        self.client
            .transport
            .request::<(), AccountPaymentsData>(
                Method::GET,
                "/v2.2/account/payments",
                &[],
                None,
                true,
            )
            .await
    }

    /// `GET /v2.2/account/registration` — current SIP registration. Rate-limited.
    pub async fn registration(&self) -> Result<AccountRegistrationData, ApiError> {
        self.client
            .transport
            .request::<(), AccountRegistrationData>(
                Method::GET,
                "/v2.2/account/registration",
                &[],
                None,
                true,
            )
            .await
    }

    /// `POST /v2.2/account/recovery` — start the password-recovery flow (no auth).
    pub async fn recover(
        &self,
        body: &AccountRecoverRequest,
    ) -> Result<AccountRecoverData, ApiError> {
        self.client
            .transport
            .request(
                Method::POST,
                "/v2.2/account/recovery",
                &[],
                Some(body),
                false,
            )
            .await
    }
}
