//! Models for the `Numbers` resource.

use serde::{Deserialize, Serialize};

/// Body for `POST /v2.2/numbers`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberAddRequest {
    /// 10-digit TN to attach.
    pub number: String,
    /// Gateway route ID; defaults to `4` (DID) on the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<i32>,
}

/// Body for `PUT /v2.2/numbers/{number}/route`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberRouteRequest {
    /// New route ID.
    pub route: i32,
}

/// Body for `PUT /v2.2/numbers/{number}/cnam`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberCnamRequest {
    /// `true` to enable inbound CNAM dips.
    pub enabled: bool,
}

/// Body for `PUT /v2.2/numbers/{number}/lidb`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberLidbRequest {
    /// Outbound caller name; max 15 alphanumeric chars.
    pub cnam: String,
    /// Optional carrier reference.
    #[serde(
        default,
        rename = "customerOrderReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_order_reference: Option<String>,
}

/// Body for `PUT /v2.2/numbers/{number}/fax`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberFaxRequest {
    /// Destination email for inbound faxes.
    pub email: String,
}

/// Body for `PUT /v2.2/numbers/{number}/forward`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberForwardRequest {
    /// 10-digit destination TN.
    pub destination: String,
}

/// Body for `PUT /v2.2/numbers/{number}/translation`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberTranslationRequest {
    /// Digits and `#` only.
    pub translation: String,
}

/// Body for `PUT /v2.2/numbers/{number}/sms`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberSmsRequest {
    /// `email`, `webhook`, or `sip`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Email / URL / IP per `kind`.
    pub resource: String,
}

/// Body for `PATCH /v2.2/numbers/{number}/messaging`.
///
/// At least one of `route_in` / `route_out` must be set.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMessagingPatchRequest {
    /// `numbers_sms` row ID; `0` detaches.
    #[serde(default, rename = "routeIn", skip_serializing_if = "Option::is_none")]
    pub route_in: Option<i32>,
    /// Outbound carrier ID.
    #[serde(default, rename = "routeOut", skip_serializing_if = "Option::is_none")]
    pub route_out: Option<i32>,
}

/// Body for `PUT /v2.2/numbers/{number}/messaging-campaign`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberCampaignAssignRequest {
    /// 7-character TCR campaign ID (uppercase alphanumeric).
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
}

/// Body for `PATCH /v2.2/numbers/{number}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMoveRequest {
    /// Destination account ID.
    #[serde(rename = "accountId")]
    pub account_id: i64,
    /// Route ID to install on the destination account.
    pub route: i32,
}

/// Body for `PATCH /v2.2/numbers/{number}/port-out-pin`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortOutPinUpdateRequest {
    /// 4-digit numeric PIN.
    pub pin: String,
}

/// Body for `DELETE /v2.2/numbers/messaging-campaign` (bulk unassign).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkUnassignRequest {
    /// TNs to unassign.
    pub numbers: Vec<String>,
}

/// Per-number routing/feature state returned by `GET /v2.2/numbers` and
/// `GET /v2.2/numbers/{number}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberDetail {
    /// 10-digit TN.
    pub number: String,
    /// DNIS translation in effect.
    pub translated: String,
    /// Gateway route ID.
    pub route: i32,
    /// Gateway IP/hostname, if any.
    pub gateway: Option<String>,
    /// `true` while inbound CNAM lookup is enabled.
    pub cnam: bool,
    /// `true` while call forwarding is enabled.
    pub forward: bool,
    /// Forward destination, when enabled.
    #[serde(rename = "forwardTo")]
    pub forward_to: Option<String>,
    /// Outbound messaging carrier ID; `0` means none.
    pub carrier: i32,
    /// `true` when SMS is enabled on this number.
    #[serde(rename = "smsEnabled")]
    pub sms_enabled: bool,
    /// `true` when fax-to-email is enabled.
    #[serde(rename = "faxEnabled")]
    pub fax_enabled: bool,
}

/// Campaign currently bound to a number, with CSP status.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CampaignBinding {
    /// 7-character TCR campaign ID.
    pub id: String,
    /// `A` or `B`.
    pub network: String,
    /// CSP status word.
    pub status: String,
    /// Upstream CNP ID.
    #[serde(rename = "upstreamCnpId")]
    pub upstream_cnp_id: String,
}

/// Messaging-routing state for one number.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMessagingState {
    /// 10-digit TN.
    pub number: String,
    /// `true` when the TN is on the authenticated account.
    #[serde(default, rename = "onAccount", skip_serializing_if = "Option::is_none")]
    pub on_account: Option<bool>,
    /// `true` while messaging is enabled.
    pub enabled: bool,
    /// Outbound messaging carrier ID.
    pub carrier: i32,
    /// Inbound route ID.
    #[serde(rename = "routeIn")]
    pub route_in: i32,
    /// Inbound resource (email / URL / IP).
    pub resource: String,
    /// `A`, `B`, or `None`.
    pub network: Option<String>,
    /// Bound campaign, if any.
    pub campaign: Option<CampaignBinding>,
}

/// Response data for `POST /v2.2/numbers`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberAddData {
    /// 10-digit TN.
    pub number: String,
    /// Route ID that was installed.
    pub route: i32,
}

/// Response data for `PUT /v2.2/numbers/{number}/cnam`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberCnamData {
    /// 10-digit TN.
    pub number: String,
    /// Resulting CNAM state.
    pub cnam: bool,
}

/// Response data for `GET`/`PUT /v2.2/numbers/{number}/fax`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberFaxData {
    /// 10-digit TN.
    pub number: String,
    /// Destination email for inbound faxes.
    pub email: String,
}

/// Response data for `PUT /v2.2/numbers/{number}/forward`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberForwardData {
    /// 10-digit TN.
    pub number: String,
    /// 10-digit forward destination, or `None` when disabled.
    #[serde(rename = "forwardTo")]
    pub forward_to: Option<String>,
}

/// Response data for `PUT /v2.2/numbers/{number}/lidb`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberLidbData {
    /// 10-digit TN.
    pub number: String,
    /// Sanitised caller name (max 15).
    pub cnam: String,
    /// Echoed or auto-generated reference.
    #[serde(rename = "customerOrderReference")]
    pub customer_order_reference: String,
    /// Carrier status word.
    #[serde(rename = "carrierStatus")]
    pub carrier_status: String,
}

/// Response data for `PATCH /v2.2/numbers/{number}/messaging`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMessagingPatchData {
    /// 10-digit TN.
    pub number: String,
    /// Subset of `["routeIn", "routeOut"]` that the server applied.
    pub updated: Vec<String>,
}

/// Response data for `PATCH /v2.2/numbers/{number}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMoveData {
    /// 10-digit TN.
    pub number: String,
    /// Destination account ID.
    #[serde(rename = "accountId")]
    pub account_id: i64,
    /// Route ID installed on the destination.
    pub route: i32,
}

/// Response data for `PUT /v2.2/numbers/{number}/route`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberRouteData {
    /// 10-digit TN.
    pub number: String,
    /// Route ID that was installed.
    pub route: i32,
}

/// Response data for `GET`/`PUT /v2.2/numbers/{number}/sms`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberSmsData {
    /// 10-digit TN.
    pub number: String,
    /// `email`, `webhook`, `sip`, or `unknown`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Email / URL / IP per `kind`.
    pub resource: String,
}

/// Response data for `PUT /v2.2/numbers/{number}/translation`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberTranslationData {
    /// 10-digit TN.
    pub number: String,
    /// Translation applied.
    pub translation: String,
}

/// Response data for `PUT /v2.2/numbers/{number}/messaging-campaign`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMessagingCampaignAssignData {
    /// 10-digit TN.
    pub number: String,
    /// TCR campaign ID.
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// `17` = path A, `19` = path B.
    pub carrier: i32,
    /// `A`, `B`, or `None`.
    pub network: Option<String>,
    /// `SFL9UTQ` for path A, `SB8TWLO` for path B.
    #[serde(rename = "upstreamCnpId")]
    pub upstream_cnp_id: Option<String>,
    /// `A`, `B`, `unknown`, or `None`.
    #[serde(rename = "previousNetwork")]
    pub previous_network: Option<String>,
    /// `true` if a prior binding was disabled.
    #[serde(rename = "previousNetworkCleared")]
    pub previous_network_cleared: bool,
}

/// Response data for `DELETE /v2.2/numbers/{number}/messaging-campaign`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberMessagingCampaignUnassignData {
    /// 10-digit TN.
    pub number: String,
    /// TCR campaign ID that was unbound.
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// `A`, `B`, or `None`.
    pub network: Option<String>,
    /// Upstream CNP ID.
    #[serde(rename = "upstreamCnpId")]
    pub upstream_cnp_id: Option<String>,
    /// Always `true` on 200.
    pub unassigned: bool,
}

/// One row in [`NumbersMessagingCampaignUnassignData::failed`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CampaignUnassignFailure {
    /// TN that failed.
    pub number: String,
    /// Server-reported reason.
    pub reason: String,
}

/// Response data for `DELETE /v2.2/numbers/messaging-campaign` (bulk unassign).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumbersMessagingCampaignUnassignData {
    /// Campaign ID that was unbound.
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// `A`, `B`, or `None`.
    pub network: Option<String>,
    /// Upstream CNP ID.
    #[serde(rename = "upstreamCnpId")]
    pub upstream_cnp_id: Option<String>,
    /// TNs that were successfully unassigned.
    #[serde(rename = "unassignedNumbers")]
    pub unassigned_numbers: Vec<String>,
    /// TNs that failed, with reason.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<CampaignUnassignFailure>,
}

/// Response data for `GET /v2.2/numbers`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumbersListData {
    /// TNs on the account.
    pub numbers: Vec<NumberDetail>,
}

/// Response data for `GET /v2.2/numbers/messaging`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumbersMessagingListData {
    /// Per-number messaging state.
    pub numbers: Vec<NumberMessagingState>,
}

/// Response data for `PATCH /v2.2/numbers/{number}/port-out-pin`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortOutPinUpdateData {
    /// 10-digit TN.
    pub number: String,
    /// Newly-set PIN.
    #[serde(rename = "portOutPin")]
    pub port_out_pin: String,
}
