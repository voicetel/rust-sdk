//! Models for the `Numbers` resource.

use serde::{Deserialize, Serialize};

/// Body for `POST /v2.2/numbers`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberAddRequest {
    /// 10-digit TN to attach.
    pub number: String,
    /// Global route type: `1` USER, `2` T30, `3` VOICE_BRIDGE, `4` DID
    /// (default), `5` T38, `6` VOICEML, `8` SDN, `9` MOBILE. Defaults to `4`
    /// (DID) on the server when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<i32>,
}

/// Body for `PUT /v2.2/numbers/{number}/route`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumberRouteRequest {
    /// Global route type to install: `1` USER, `2` T30, `3` VOICE_BRIDGE,
    /// `4` DID (default), `5` T38, `6` VOICEML, `8` SDN, `9` MOBILE.
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
    /// Outbound carrier id.
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
    /// Route on the destination account after the move — global route type:
    /// `1` USER, `2` T30, `3` VOICE_BRIDGE, `4` DID (default), `5` T38,
    /// `6` VOICEML, `8` SDN, `9` MOBILE.
    pub route: i32,
}

/// Body for `PATCH /v2.2/numbers/{number}/port-out-pin`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortOutPinUpdateRequest {
    /// 4-digit numeric PIN.
    pub pin: String,
}

/// Per-number routing/feature state returned by `GET /v2.2/numbers` and
/// `GET /v2.2/numbers/{number}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberDetail {
    /// 10-digit TN.
    pub number: String,
    /// DNIS translation in effect.
    pub translated: String,
    /// Global route type: `1` USER, `2` T30, `3` VOICE_BRIDGE, `4` DID
    /// (default), `5` T38, `6` VOICEML, `8` SDN, `9` MOBILE.
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
#[serde(default)]
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

/// Lifecycle state of a [`MessagingEnablementJob`]: `queued` → `hosting` →
/// `assigning` → `routing` → `done`, or `failed` (see `message`). The number
/// keeps its previous routing until `done`.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessagingEnablementJobState {
    /// Accepted, waiting to start.
    #[default]
    #[serde(rename = "queued")]
    Queued,
    /// Being hosted on the messaging path.
    #[serde(rename = "hosting")]
    Hosting,
    /// Campaign assignment in flight.
    #[serde(rename = "assigning")]
    Assigning,
    /// Routing being installed.
    #[serde(rename = "routing")]
    Routing,
    /// Provisioning complete.
    #[serde(rename = "done")]
    Done,
    /// Provisioning failed (see [`MessagingEnablementJob::message`]).
    #[serde(rename = "failed")]
    Failed,
}

/// State of an asynchronous (path C) 10DLC enablement. Returned by
/// `PUT /v2.2/numbers/{number}/messaging-campaign` (`status` `pending`) and
/// surfaced by `GET /v2.2/numbers/{number}/messaging` (`pending`) while
/// provisioning is in flight.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MessagingEnablementJob {
    /// TCR campaign ID being enabled.
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// Job state.
    pub state: MessagingEnablementJobState,
    /// Latest progress note, or the reason for failure.
    pub message: String,
    /// RFC3339 timestamp the job was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// RFC3339 timestamp of the latest progress update.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Messaging-routing state for one number.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
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
    /// `A`, `B`, `C`, or `None`.
    pub network: Option<String>,
    /// Bound campaign, if any.
    pub campaign: Option<CampaignBinding>,
    /// Async path-C enablement in progress, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<MessagingEnablementJob>,
}

/// Response data for `POST /v2.2/numbers`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberAddData {
    /// 10-digit TN.
    pub number: String,
    /// Global route type that was installed: `1` USER, `2` T30,
    /// `3` VOICE_BRIDGE, `4` DID (default), `5` T38, `6` VOICEML, `8` SDN,
    /// `9` MOBILE.
    pub route: i32,
}

/// Response data for `PUT /v2.2/numbers/{number}/cnam`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberCnamData {
    /// 10-digit TN.
    pub number: String,
    /// Resulting CNAM state.
    pub cnam: bool,
}

/// Response data for `GET`/`PUT /v2.2/numbers/{number}/fax`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberFaxData {
    /// 10-digit TN.
    pub number: String,
    /// Destination email for inbound faxes.
    pub email: String,
}

/// Response data for `PUT /v2.2/numbers/{number}/forward`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberForwardData {
    /// 10-digit TN.
    pub number: String,
    /// 10-digit forward destination, or `None` when disabled.
    #[serde(rename = "forwardTo")]
    pub forward_to: Option<String>,
}

/// Response data for `PUT /v2.2/numbers/{number}/lidb`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
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
#[serde(default)]
pub struct NumberMessagingPatchData {
    /// 10-digit TN.
    pub number: String,
    /// Subset of `["routeIn", "routeOut"]` that the server applied.
    pub updated: Vec<String>,
}

/// Response data for `PATCH /v2.2/numbers/{number}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberMoveData {
    /// 10-digit TN.
    pub number: String,
    /// Destination account ID.
    #[serde(rename = "accountId")]
    pub account_id: i64,
    /// Global route type installed on the destination: `1` USER, `2` T30,
    /// `3` VOICE_BRIDGE, `4` DID (default), `5` T38, `6` VOICEML, `8` SDN,
    /// `9` MOBILE.
    pub route: i32,
}

/// Response data for `PUT /v2.2/numbers/{number}/route`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberRouteData {
    /// 10-digit TN.
    pub number: String,
    /// Echo of the applied global route type: `1` USER, `2` T30,
    /// `3` VOICE_BRIDGE, `4` DID (default), `5` T38, `6` VOICEML, `8` SDN,
    /// `9` MOBILE.
    pub route: i32,
}

/// Response data for `GET`/`PUT /v2.2/numbers/{number}/sms`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
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
#[serde(default)]
pub struct NumberTranslationData {
    /// 10-digit TN.
    pub number: String,
    /// Translation applied.
    pub translation: String,
}

/// Outcome of binding a number to a 10DLC campaign
/// (`PUT /v2.2/numbers/{number}/messaging-campaign`).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum CampaignAssignStatus {
    /// Bound now (path B).
    #[default]
    #[serde(rename = "enabled")]
    Enabled,
    /// The number was already bound to this campaign (records reconciled,
    /// no new order).
    #[serde(rename = "already")]
    Already,
    /// Accepted and being provisioned (path C) — poll
    /// `GET /v2.2/numbers/{number}/messaging` (`pending`) until the job's
    /// state is `done` or `failed`.
    #[serde(rename = "pending")]
    Pending,
}

/// Network order ID — string or integer on the wire, or `None`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum OrderId {
    /// String form.
    Text(String),
    /// Numeric form.
    Numeric(i64),
}

/// Response data for `PUT /v2.2/numbers/{number}/messaging-campaign`.
///
/// Result of binding a single number to a 10DLC campaign. The campaign must
/// be available to the account and ACTIVE; the number is enabled on the
/// messaging path the campaign lives on — path B synchronously (`status`
/// `enabled`), path C asynchronously (`status` `pending`: poll
/// `GET /v2.2/numbers/{number}/messaging` `pending` until the job's `state`
/// is `done` or `failed`). Idempotent: a number already bound to the campaign
/// reports `status` `already`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumberMessagingCampaignAssignData {
    /// 10-digit TN.
    pub number: String,
    /// TCR campaign ID.
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// Assignment outcome.
    pub status: CampaignAssignStatus,
    /// Outbound carrier id the number routes through once enabled
    /// (`19` = path B, `1` = path C). For a pending path-C enablement the
    /// number keeps its current routing until provisioning completes.
    pub carrier: i32,
    /// Messaging path the campaign resolves to (from the CSP sharing record):
    /// `B`, `C`, or `None`. Path B is assigned synchronously; path C is
    /// provisioned asynchronously (`status` `pending`).
    pub network: Option<String>,
    /// CNP ID from the CSP sharing record: `CAMP003` = path B, `BANDW` =
    /// path C. (Campaigns on path A, `CAMP004`, are being migrated and take
    /// no new numbers.)
    #[serde(rename = "upstreamCnpId")]
    pub upstream_cnp_id: Option<String>,
    /// Messaging path the number was bound to before this call: `A`, `B`, or
    /// `None` if there was no prior binding.
    #[serde(rename = "previousNetwork")]
    pub previous_network: Option<String>,
    /// `true` if a prior binding was disabled.
    #[serde(rename = "previousNetworkCleared")]
    pub previous_network_cleared: bool,
    /// `true` when the number was already bound to this campaign on the
    /// target messaging path; records are reconciled and no new network order
    /// is placed.
    #[serde(rename = "alreadyAssigned")]
    pub already_assigned: bool,
    /// Network order id created for a path-B assignment. `None` for path C
    /// (see `job`), idempotent no-ops, and assignments confirmed without a
    /// new order.
    #[serde(rename = "orderId")]
    pub order_id: Option<OrderId>,
    /// Non-fatal condition detected during the assignment (for example a
    /// stale registration on the other messaging path whose removal has been
    /// requested).
    pub warning: Option<String>,
    /// Human-readable note for a pending enablement.
    pub message: Option<String>,
    /// Async path-C enablement job, when `status` is `pending`.
    pub job: Option<MessagingEnablementJob>,
}

/// Response data for `DELETE /v2.2/numbers/{number}/messaging-campaign`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CampaignUnassignFailure {
    /// TN that failed.
    pub number: String,
    /// Server-reported reason.
    pub reason: String,
}

/// Response data for `DELETE /v2.2/numbers/messaging-campaign` (bulk unassign).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
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
#[serde(default)]
pub struct NumbersListData {
    /// TNs on the account.
    pub numbers: Vec<NumberDetail>,
}

/// Response data for `GET /v2.2/numbers/messaging`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NumbersMessagingListData {
    /// Per-number messaging state.
    pub numbers: Vec<NumberMessagingState>,
}

/// Response data for `PATCH /v2.2/numbers/{number}/port-out-pin`.
///
/// The call is idempotent: a PIN is locked for 30 days once set, so a number
/// that already has one returns it unchanged with `changed == false`; both
/// cases return HTTP 200.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PortOutPinUpdateData {
    /// Ten-digit TN whose PIN is reported.
    pub number: String,
    /// The port-out PIN now on file for this number (the value just set, or
    /// the existing value if the number was already locked).
    #[serde(rename = "portOutPin")]
    pub port_out_pin: String,
    /// `true` if this request set the PIN; `false` if the number already had
    /// one, which is locked for 30 days once set and is returned unchanged.
    pub changed: bool,
}
