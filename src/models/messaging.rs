//! Models for the `Messaging` resource.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Body for `POST /v2.2/messages`. Wire field names are camelCase
/// (`fromNumber`, `toNumber`).
///
/// Non-empty `media_urls` switches the message type to MMS and unlocks `subject`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageSendRequest {
    /// 10-digit TN on the authenticated account.
    #[serde(rename = "fromNumber")]
    pub from_number: String,
    /// 10-digit destination TN.
    #[serde(rename = "toNumber")]
    pub to_number: String,
    /// UTF-8 message body.
    pub text: String,
    /// MMS subject (MMS only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Media URLs — presence makes this an MMS.
    #[serde(default, rename = "mediaUrls", skip_serializing_if = "Vec::is_empty")]
    pub media_urls: Vec<String>,
}

/// Body for `POST /v2.2/messaging/brands`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessagingBrandCreateRequest {
    /// Brand ID, starting with `B`, alphanumeric.
    #[serde(rename = "messagingBrandId")]
    pub messaging_brand_id: String,
    /// Brand display name.
    #[serde(rename = "messagingBrandName")]
    pub messaging_brand_name: String,
    /// Free-form description.
    #[serde(
        default,
        rename = "messagingBrandDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub messaging_brand_description: Option<String>,
}

/// Body for `POST /v2.2/messaging/campaigns`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessagingCampaignCreateRequest {
    /// Owning brand ID.
    #[serde(rename = "messagingBrandId")]
    pub messaging_brand_id: String,
    /// TCR campaign ID.
    #[serde(rename = "externalCampaignId")]
    pub external_campaign_id: String,
    /// Free-form campaign description.
    #[serde(rename = "campaignDescription")]
    pub campaign_description: String,
    /// Use case class (auto-populated when omitted).
    #[serde(
        default,
        rename = "campaignClassName",
        skip_serializing_if = "Option::is_none"
    )]
    pub campaign_class_name: Option<String>,
    /// ISO-8601 start date (auto-populated when omitted).
    #[serde(
        default,
        rename = "campaignStartDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub campaign_start_date: Option<String>,
}

/// Per-record value inside a [`MessageRecord`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageRecordValue {
    /// Source TN.
    #[serde(
        default,
        rename = "sourceNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_number: Option<String>,
    /// Destination TN.
    #[serde(
        default,
        rename = "destinationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_number: Option<String>,
    /// `in` or `out` (sms/mms only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Billed rate per message (string for exact precision).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Far-end number (sms/mms only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// Message body (sms/mms only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// One row in [`MessageHistoryData::messages`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageRecord {
    /// Per-row identifier.
    pub id: String,
    /// Mixed-type key array.
    pub key: Vec<Value>,
    /// Per-row value.
    pub value: MessageRecordValue,
}

/// Response data for `GET /v2.2/messages`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageHistoryData {
    /// Number filter echoed back.
    pub number: String,
    /// `sms`, `mms`, or `dlr`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Echo of the `from` query parameter.
    #[serde(rename = "fromTs")]
    pub from_ts: i64,
    /// Echo of the `to` query parameter.
    #[serde(rename = "toTs")]
    pub to_ts: i64,
    /// Message rows.
    pub messages: Vec<MessageRecord>,
}

/// Response data for `POST /v2.2/messages`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageSendData {
    /// Provider transaction ID.
    pub id: String,
    /// `sms` or `mms`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Source TN.
    #[serde(rename = "fromNumber")]
    pub from_number: String,
    /// Destination TN.
    #[serde(rename = "toNumber")]
    pub to_number: String,
    /// Billed SMS segments; `1` for MMS.
    pub parts: i32,
    /// MMS subject.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// MMS media URLs.
    #[serde(default, rename = "mediaUrls", skip_serializing_if = "Vec::is_empty")]
    pub media_urls: Vec<String>,
}

/// Status payload for brand registration.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrandRegistrationResult {
    /// HTTP status code as string (`200` on success).
    #[serde(rename = "statusCode")]
    pub status_code: String,
    /// `Success` on success.
    pub status: String,
}

/// Response data for `POST /v2.2/messaging/brands`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessagingBrandCreateData {
    /// Status payload.
    pub result: BrandRegistrationResult,
}

/// Status payload for campaign registration.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CampaignRegistrationResult {
    /// HTTP status code as string.
    #[serde(rename = "statusCode")]
    pub status_code: String,
    /// Status word.
    pub status: String,
}

/// Response data for `POST /v2.2/messaging/campaigns`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessagingCampaignCreateData {
    /// Status payload.
    pub result: CampaignRegistrationResult,
}

/// One campaign and its currently-bound numbers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CampaignStatusItem {
    /// 7-character TCR campaign ID.
    pub id: String,
    /// CSP status word.
    pub status: String,
    /// TNs currently bound.
    pub numbers: Vec<String>,
}

/// Response data for `GET /v2.2/messaging/campaigns`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessagingCampaignStatusData {
    /// Per-campaign rows.
    pub campaigns: Vec<CampaignStatusItem>,
}

/// Optional query filters for `MessagingService::history`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HistoryOptions {
    /// 10-digit TN whose history to fetch.
    pub number: Option<String>,
    /// Unix-seconds range start.
    pub start: Option<i64>,
    /// Unix-seconds range end (older than `start`).
    pub end: Option<i64>,
    /// `sms`, `mms`, or `dlr`.
    pub kind: Option<String>,
}
