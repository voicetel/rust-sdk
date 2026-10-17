//! `Messaging` resource.

use reqwest::Method;
use serde_json::{json, Value};

use crate::client::Client;
use crate::error::ApiError;
use crate::models::messaging::*;
use crate::models::numbers::NumbersMessagingListData;

/// Handle for the `Messaging` resource — SMS/MMS send + 10DLC.
#[derive(Debug)]
pub struct MessagingService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> MessagingService<'c> {
    /// `GET /v2.2/messages` — message history for one number. `number` is
    /// required by the server; `from`/`to` bound the inclusive Unix-timestamp
    /// range, and `kind` selects `sms` (default), `mms`, or `dlr`.
    pub async fn history(&self, opts: &HistoryOptions) -> Result<MessageHistoryData, ApiError> {
        let mut q: Vec<(&str, String)> = vec![("number", opts.number.clone())];
        if let Some(f) = opts.from {
            q.push(("from", f.to_string()));
        }
        if let Some(t) = opts.to {
            q.push(("to", t.to_string()));
        }
        if let Some(ref t) = opts.kind {
            q.push(("type", t.clone()));
        }
        self.client
            .transport
            .request::<(), MessageHistoryData>(Method::GET, "/v2.2/messages", &q, None, true)
            .await
    }

    /// `POST /v2.2/messages` — send an SMS or MMS.
    pub async fn send(&self, body: &MessageSendRequest) -> Result<MessageSendData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/messages", &[], Some(body), true)
            .await
    }

    /// `POST /v2.2/messaging/brands` — register a 10DLC brand.
    pub async fn create_brand(
        &self,
        body: &MessagingBrandCreateRequest,
    ) -> Result<MessagingBrandCreateData, ApiError> {
        self.client
            .transport
            .request(
                Method::POST,
                "/v2.2/messaging/brands",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// `GET /v2.2/messaging/campaigns` — per-campaign status and bound numbers
    /// for the given comma-joined campaign ids (max 100; required).
    pub async fn campaign_status(
        &self,
        ids: &[String],
    ) -> Result<MessagingCampaignStatusData, ApiError> {
        let q = vec![("ids", ids.join(","))];
        self.client
            .transport
            .request::<(), MessagingCampaignStatusData>(
                Method::GET,
                "/v2.2/messaging/campaigns",
                &q,
                None,
                true,
            )
            .await
    }

    /// `POST /v2.2/messaging/campaigns` — register a 10DLC campaign.
    pub async fn create_campaign(
        &self,
        body: &MessagingCampaignCreateRequest,
    ) -> Result<MessagingCampaignCreateData, ApiError> {
        self.client
            .transport
            .request(
                Method::POST,
                "/v2.2/messaging/campaigns",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// `GET /v2.2/numbers/messaging` — messaging state for the given numbers
    /// (comma-joined; required, max 1000). Numbers not on the authenticated
    /// account come back with `on_account == Some(false)`.
    pub async fn numbers_state(
        &self,
        numbers: &[String],
    ) -> Result<NumbersMessagingListData, ApiError> {
        let q = vec![("numbers", numbers.join(","))];
        self.client
            .transport
            .request::<(), NumbersMessagingListData>(
                Method::GET,
                "/v2.2/numbers/messaging",
                &q,
                None,
                true,
            )
            .await
    }

    /// `GET /v2.2/messaging/health` — account-wide messaging health overview.
    ///
    /// Mirrors the portal Messaging tab: summary counts (numbers, messaging on,
    /// healthy, needing attention, in progress), campaigns with carrier-review
    /// progress, numbers needing attention (each with a plain-language title, detail
    /// and the actions that fix it), 14 days of delivery counts, and the alert-email
    /// setting. Updated hourly.
    ///
    /// The API spec defines no response schema for this endpoint, so the decoded JSON
    /// is returned as a raw [`Value`] rather than a typed model.
    pub async fn health(&self) -> Result<Value, ApiError> {
        self.client
            .transport
            .request::<(), Value>(Method::GET, "/v2.2/messaging/health", &[], None, true)
            .await
    }

    /// `GET /v2.2/messaging/health/alerts` — the alert-email setting: whether new
    /// messaging problems are emailed to the account.
    ///
    /// The spec defines no response schema, so the decoded JSON is returned raw.
    pub async fn health_alerts(&self) -> Result<Value, ApiError> {
        self.client
            .transport
            .request::<(), Value>(
                Method::GET,
                "/v2.2/messaging/health/alerts",
                &[],
                None,
                true,
            )
            .await
    }

    /// `PUT /v2.2/messaging/health/alerts` — turn alert emails on or off.
    ///
    /// When enabled, the account is emailed as new messaging problems are found (at
    /// most one digest about every 20 hours). The spec defines no response schema, so
    /// the decoded JSON is returned raw.
    pub async fn set_health_alerts(&self, enabled: bool) -> Result<Value, ApiError> {
        let body = json!({ "enabled": enabled });
        self.client
            .transport
            .request(
                Method::PUT,
                "/v2.2/messaging/health/alerts",
                &[],
                Some(&body),
                true,
            )
            .await
    }

    /// `POST /v2.2/messaging/health/check` — live health check of one number.
    ///
    /// Reports in service, messaging enabled, campaign approved, assigned to the
    /// campaign by the messaging provider, delivery failures in the last 24 hours, and
    /// enablement in progress. Limited to 30 checks per hour per account. The spec
    /// defines no response schema, so the decoded JSON is returned raw.
    pub async fn health_check(&self, number: &str) -> Result<Value, ApiError> {
        let body = json!({ "number": number });
        self.client
            .transport
            .request(
                Method::POST,
                "/v2.2/messaging/health/check",
                &[],
                Some(&body),
                true,
            )
            .await
    }

    /// `GET /v2.2/messaging/health/samples` — recent messages blocked as spam.
    ///
    /// Up to 3 recent outbound messages from `number` that carriers blocked as spam
    /// (time, masked recipient, text, reason), to help review content. The spec defines
    /// no response schema, so the decoded JSON is returned raw.
    pub async fn health_samples(&self, number: &str) -> Result<Value, ApiError> {
        let q = vec![("number", number.to_string())];
        self.client
            .transport
            .request::<(), Value>(
                Method::GET,
                "/v2.2/messaging/health/samples",
                &q,
                None,
                true,
            )
            .await
    }
}
