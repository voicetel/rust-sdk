//! Models for the `Account` resource.

use serde::{Deserialize, Serialize};

/// Per-service rates exposed on an account. Read-only for non-admin callers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountRates {
    /// CNAM dip rate (USD per dip).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnam: Option<f64>,
    /// International call cap (USD/min).
    #[serde(default, rename = "intlMax", skip_serializing_if = "Option::is_none")]
    pub intl_max: Option<f64>,
    /// Per-second outbound rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nibble: Option<f64>,
    /// LRN dip rate (USD per dip).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lrn: Option<f64>,
    /// Fax page rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<f64>,
    /// Toll-free adjusted rate.
    #[serde(default, rename = "tfAdj", skip_serializing_if = "Option::is_none")]
    pub tf_adj: Option<f64>,
    /// Per-DID monthly recurring rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did: Option<f64>,
    /// MMS rate (USD per message).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mms: Option<f64>,
    /// SMS rate (USD per segment).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<f64>,
}

/// Per-service feature flags. `true` means the service is enabled on the account.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountServices {
    /// e911 provisioning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e911: Option<bool>,
    /// CNAM dips.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnam: Option<bool>,
    /// SIP media bypass.
    #[serde(
        default,
        rename = "bypassMedia",
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_media: Option<bool>,
    /// International calling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intl: Option<bool>,
    /// Remote Caller-ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcid: Option<bool>,
    /// MMS delivery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mms: Option<bool>,
    /// Predictive dialer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialer: Option<bool>,
    /// SMS delivery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
}

/// Profile returned by `GET /v2.2/account`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountData {
    /// Account username (numeric string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// `true` while the account is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// ISO-8601 creation timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Current account balance, USD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<f64>,
    /// Default outbound caller-ID.
    #[serde(default, rename = "callerId", skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,
    /// IANA tz database identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Authentication mode (see [`crate::resources::authentication`] constants).
    #[serde(default, rename = "authType", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<i32>,
    /// Concurrent-call ceiling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccs: Option<i32>,
    /// Whether low-balance notifications are enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    /// Threshold (USD) below which to fire a notification.
    #[serde(
        default,
        rename = "notifyThreshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify_threshold: Option<i32>,
    /// Per-service rates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<AccountRates>,
    /// Per-service flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<AccountServices>,
}

/// Body for `POST /v2.2/account` (admin-only sub-account creation).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountAddRequest {
    /// Numeric username (e.g. `1000000123`).
    pub username: u64,
    /// Display name.
    pub name: String,
    /// Contact email.
    pub email: String,
    /// Optional master account ID this sub-account rolls up to.
    #[serde(rename = "masterAccount", skip_serializing_if = "Option::is_none")]
    pub master_account: Option<u64>,
}

/// Response data for `POST /v2.2/account`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountAddData {
    /// New account username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Master account, when configured.
    #[serde(
        default,
        rename = "masterAccount",
        skip_serializing_if = "Option::is_none"
    )]
    pub master_account: Option<String>,
    /// Auto-generated initial password.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Body for `PUT /v2.2/account`. All fields optional; `None` = leave unchanged.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountPutRequest {
    /// Toggle low-balance notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    /// Notification threshold (USD).
    #[serde(rename = "notifyThreshold", skip_serializing_if = "Option::is_none")]
    pub notify_threshold: Option<i32>,
    /// Account timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Default outbound caller-ID.
    #[serde(rename = "callerId", skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,
    /// Toggle e911 (admin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e911: Option<bool>,
    /// Toggle international calling (admin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intl: Option<bool>,
    /// Toggle SMS (admin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
    /// Toggle MMS (admin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mms: Option<bool>,
    /// Concurrent-call ceiling (admin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccs: Option<i32>,
}

/// Response data for `PUT /v2.2/account`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountPutData {
    /// Fields the server actually updated.
    pub updated: Vec<String>,
}

/// Body for `POST /v2.2/accounts` (public sign-up).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountSignupRequest {
    /// Display name.
    pub name: String,
    /// Contact email.
    pub email: String,
    /// Optional promo code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo: Option<String>,
}

/// Response data for `POST /v2.2/accounts`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountSignupData {
    /// Newly-issued username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Display name echoed back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Email echoed back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Initial password.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Body for `POST /v2.2/account/recovery` (no auth required).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountRecoverRequest {
    /// Email registered on the account.
    pub email: String,
}

/// Response data for `POST /v2.2/account/recovery`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountRecoverData {
    /// Confirmation message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Response data for `POST /v2.2/account/api-key`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountApiKeyData {
    /// 32-hex bearer token.
    pub apikey: String,
}

/// One credit row in [`AccountCreditsData`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreditEntry {
    /// ISO-8601 date the credit was applied.
    pub date: String,
    /// `true` once the credit has been settled.
    pub paid: bool,
    /// Credit amount in USD.
    pub amount: f64,
}

/// One payment row in [`AccountPaymentsData`].
///
/// `status` is one of `Completed`, `Pending`, `Reversed`, `Refunded`, `Failed`,
/// `Denied`, `Canceled_Reversal`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PaymentEntry {
    /// Provider transaction ID.
    #[serde(
        default,
        rename = "transactionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_id: Option<String>,
    /// ISO-8601 date the payment was attempted.
    pub date: String,
    /// PayPal/Stripe payer email.
    #[serde(
        default,
        rename = "payerEmail",
        skip_serializing_if = "Option::is_none"
    )]
    pub payer_email: Option<String>,
    /// Status word.
    pub status: String,
    /// Payment amount in USD.
    pub amount: f64,
}

/// Per-call billing summary inside a CDR row. Numeric-looking fields are
/// intentionally strings to preserve exact precision on the wire.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CdrEntryValue {
    /// Billed duration (seconds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dur: Option<String>,
    /// Destination 10-digit TN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dst: Option<String>,
    /// Billed amount (USD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ba: Option<String>,
    /// Nibble rate (USD/min).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nr: Option<String>,
    /// URL-encoded display name (CNAM at call time).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    /// IPv4 of the SIP leg.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Caller-ID 10-digit TN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
}

/// One row in [`AccountCdrData::cdr`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CdrEntry {
    /// Per-row identifier.
    pub id: String,
    /// `[accountUsername, startEpochUnixSeconds]`.
    pub key: Vec<String>,
    /// Per-call billing summary.
    pub value: CdrEntryValue,
}

/// Response data for `GET /v2.2/account/cdr`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountCdrData {
    /// CDR rows, oldest first.
    pub cdr: Vec<CdrEntry>,
    /// Echo of the `start` query parameter (unix seconds).
    pub start: i64,
    /// Echo of the `end` query parameter (unix seconds).
    pub end: i64,
}

/// Response data for `GET /v2.2/account/credits`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountCreditsData {
    /// Credit history, newest first.
    pub credits: Vec<CreditEntry>,
}

/// Response data for `GET /v2.2/account/payments`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountPaymentsData {
    /// Payment history, newest first.
    pub payments: Vec<PaymentEntry>,
}

/// One monthly-recurring charge row inside [`AccountMrcData`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MrcCharge {
    /// Amount (USD).
    pub amount: f64,
    /// Free-form description (e.g. `DID 12015551234`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response data for `GET /v2.2/account/recurring-charges`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountMrcData {
    /// Active recurring charges.
    pub charges: Vec<MrcCharge>,
    /// Total of all charges (USD).
    pub total: f64,
}

/// Response data for `GET /v2.2/account/registration`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountRegistrationData {
    /// SIP user-agent string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Contact URI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Registration TTL (seconds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
}
