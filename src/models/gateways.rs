//! Models for the `Gateways` resource.

use serde::{Deserialize, Serialize};

/// Body for `POST /v2.2/gateways`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GatewayAddRequest {
    /// IP/hostname with optional `:port`. Must be a routable public IPv4.
    pub gateway: String,
    /// Digits to prepend on outbound calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Max concurrent calls; default 23, range 1..1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Body for `PUT /v2.2/gateways/{id}`. All fields optional.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GatewayUpdateRequest {
    /// Replacement IP/hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Replacement prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Replacement concurrent-call cap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// One gateway row. `limit` is `Option<i32>` to distinguish "unset on system
/// routes" (`None`) from `0` (which would never be valid).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GatewayEntry {
    /// Server-assigned gateway ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// IP/hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Digit prefix.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Concurrent-call cap; `None` on system routes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// `true` for system-provided routes (cannot be edited).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

/// One number bound to a gateway, returned by `GET /v2.2/gateways/{id}/numbers`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GatewayNumberSummary {
    /// 10-digit TN.
    pub number: String,
    /// DNIS translation in effect.
    pub translated: String,
    /// `true` while call forwarding is enabled.
    pub forward: bool,
    /// Forward destination (10-digit TN) or `None`.
    #[serde(rename = "forwardTo")]
    pub forward_to: Option<String>,
    /// `true` while inbound CNAM lookup is enabled.
    pub cnam: bool,
    /// Outbound messaging carrier ID; `0` means none.
    pub carrier: i32,
    /// `true` when SMS is enabled on this number.
    #[serde(rename = "smsEnabled")]
    pub sms_enabled: bool,
    /// `true` when fax-to-email is enabled on this number.
    #[serde(rename = "faxEnabled")]
    pub fax_enabled: bool,
}

/// Response data for `GET /v2.2/gateways`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GatewaysListData {
    /// Every gateway on the account.
    pub gateways: Vec<GatewayEntry>,
}

/// Response data for `GET /v2.2/gateways/{id}/numbers`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GatewayNumbersData {
    /// Every number routed through this gateway.
    pub numbers: Vec<GatewayNumberSummary>,
}
