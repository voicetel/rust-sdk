//! `Messaging` resource.

use reqwest::Method;

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
    /// `GET /v2.2/messages` — message history with optional filters.
    pub async fn history(&self, opts: &HistoryOptions) -> Result<MessageHistoryData, ApiError> {
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(ref n) = opts.number {
            q.push(("number", n.clone()));
        }
        if let Some(s) = opts.start {
            q.push(("start", s.to_string()));
        }
        if let Some(e) = opts.end {
            q.push(("end", e.to_string()));
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

    /// `GET /v2.2/messaging/campaigns` — current 10DLC campaign statuses.
    pub async fn campaign_status(&self) -> Result<MessagingCampaignStatusData, ApiError> {
        self.client
            .transport
            .request::<(), MessagingCampaignStatusData>(
                Method::GET,
                "/v2.2/messaging/campaigns",
                &[],
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

    /// `GET /v2.2/numbers/messaging` — messaging state for many numbers at once.
    /// Pass an empty slice for "all numbers on the account".
    pub async fn numbers_state(
        &self,
        numbers: &[String],
    ) -> Result<NumbersMessagingListData, ApiError> {
        let mut q: Vec<(&str, String)> = Vec::new();
        if !numbers.is_empty() {
            q.push(("numbers", numbers.join(",")));
        }
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
}
