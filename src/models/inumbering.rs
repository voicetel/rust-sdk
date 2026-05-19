//! Models for the `iNumbering` resource.

use serde::de::Error as DeError;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// One entry inside [`OrderCreateRequest::numbers`].
///
/// On the wire this is either a plain JSON string TN, or a `{number, route}`
/// object. Use [`OrderNumber::plain`] / [`OrderNumber::with_route`] to construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderNumber {
    /// Plain 10-digit TN.
    Plain(String),
    /// 10-digit TN with optional gateway route override.
    WithRoute {
        /// 10-digit TN.
        number: String,
        /// Optional gateway route ID.
        route: Option<i32>,
    },
}

impl OrderNumber {
    /// Build a plain TN entry.
    pub fn plain(number: impl Into<String>) -> Self {
        Self::Plain(number.into())
    }

    /// Build a TN entry with an explicit route ID.
    pub fn with_route(number: impl Into<String>, route: i32) -> Self {
        Self::WithRoute {
            number: number.into(),
            route: Some(route),
        }
    }
}

impl Serialize for OrderNumber {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Plain(v) => s.serialize_str(v),
            Self::WithRoute { number, route } => {
                let len = if route.is_some() { 2 } else { 1 };
                let mut st = s.serialize_struct("OrderNumberSpec", len)?;
                st.serialize_field("number", number)?;
                if let Some(r) = route {
                    st.serialize_field("route", r)?;
                }
                st.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for OrderNumber {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        match v {
            serde_json::Value::String(s) => Ok(Self::Plain(s)),
            serde_json::Value::Object(map) => {
                let number = map
                    .get("number")
                    .and_then(|x| x.as_str())
                    .ok_or_else(|| D::Error::custom("OrderNumber: missing `number`"))?
                    .to_string();
                let route = map.get("route").and_then(|x| x.as_i64()).map(|n| n as i32);
                Ok(Self::WithRoute { number, route })
            }
            _ => Err(D::Error::custom("OrderNumber: expected string or object")),
        }
    }
}

/// Body for `POST /v2.2/orders`.
///
/// `numbers` must hold 1..100 entries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderCreateRequest {
    /// TNs to purchase.
    pub numbers: Vec<OrderNumber>,
}

/// LIDB feature for a port-in TN.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortFeatureLidb {
    /// Outbound caller name; max 15 characters.
    pub name: String,
}

/// Routing feature for a port-in TN.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortFeatureRouting {
    /// Gateway ID to route this TN to once the port completes.
    #[serde(rename = "gatewayId")]
    pub gateway_id: i64,
}

/// SMS feature for a port-in TN.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortFeatureSms {
    /// TCR campaign ID to bind once the port completes.
    #[serde(
        default,
        rename = "campaignId",
        skip_serializing_if = "Option::is_none"
    )]
    pub campaign_id: Option<String>,
}

/// Per-TN feature configuration applied after a port completes.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortFeature {
    /// 10-digit TN being configured.
    pub number: String,
    /// Routing feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<PortFeatureRouting>,
    /// LIDB feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lidb: Option<PortFeatureLidb>,
    /// SMS feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<PortFeatureSms>,
}

/// Body for `POST /v2.2/ports`.
///
/// `street_prefix` / `street_suffix` are one of `N`, `NE`, `E`, `SE`, `S`,
/// `SW`, `W`, `NW`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortSubmitRequest {
    /// 10-digit TNs (toll-free not supported).
    pub did: Vec<String>,
    /// Account holder name exactly as on the losing-carrier bill.
    pub name: String,
    /// `business` or `residential`.
    #[serde(rename = "nameType")]
    pub name_type: String,
    /// Billing TN on the losing-carrier bill.
    #[serde(rename = "lcBtn")]
    pub lc_btn: String,
    /// Account number on the losing-carrier bill.
    #[serde(rename = "lcAccountNumber")]
    pub lc_account_number: String,
    /// Street number.
    #[serde(rename = "streetNumber")]
    pub street_number: String,
    /// Street name (no prefix/suffix/type).
    pub street: String,
    /// USPS abbreviation: `ST`, `AVE`, `BLVD`, …
    #[serde(rename = "streetType")]
    pub street_type: String,
    /// City.
    pub city: String,
    /// Two-letter US state code.
    pub state: String,
    /// 5- or 9-digit ZIP.
    pub zip: String,
    /// ISO 3166-1 alpha-2 country code.
    pub country: String,
    /// Full name authorised to sign the LOA.
    #[serde(rename = "authPerson")]
    pub auth_person: String,
    /// Optional directional prefix.
    #[serde(
        default,
        rename = "streetPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub street_prefix: Option<String>,
    /// Optional directional suffix.
    #[serde(
        default,
        rename = "streetSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub street_suffix: Option<String>,
    /// Floor designator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    /// Room designator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    /// Building designator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    /// Unit designator (e.g. `APT 3`, `STE 200`).
    #[serde(default, rename = "unitValue", skip_serializing_if = "Option::is_none")]
    pub unit_value: Option<String>,
    /// ISO-8601 desired due date; blank = standard SLA.
    #[serde(
        default,
        rename = "desiredDueDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_due_date: Option<String>,
    /// Port-out PIN from the losing carrier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    /// Per-TN feature configuration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<PortFeature>,
}

/// One TN available for assignment.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryItem {
    /// 10-digit TN.
    pub number: String,
    /// Rate center.
    #[serde(rename = "rateCenter")]
    pub rate_center: String,
    /// City.
    pub city: String,
    /// Two-letter state/province code.
    pub province: String,
    /// LATA.
    pub lata: String,
}

/// One aggregated availability bucket. Which fields are populated depends on
/// the `countBy` dimension supplied in the query.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryCoverageItem {
    /// Bucket count.
    pub count: i64,
    /// NPA (area code).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npa: Option<String>,
    /// NXX.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nxx: Option<String>,
    /// Block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Rate-center abbreviation.
    #[serde(default, rename = "rcAbbre", skip_serializing_if = "Option::is_none")]
    pub rc_abbre: Option<String>,
    /// LATA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lata: Option<String>,
    /// Location state.
    #[serde(default, rename = "locState", skip_serializing_if = "Option::is_none")]
    pub loc_state: Option<String>,
}

/// One row in the port-status list.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortSummary {
    /// Port status word.
    pub status: String,
    /// Internal port ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 5-character port order ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// Firm Order Commitment date (`YYYYMMDD`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foc: Option<String>,
    /// ISO-8601 creation timestamp.
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Carrier message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Support URL.
    #[serde(
        default,
        rename = "supportUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub support_url: Option<String>,
}

/// Full record for a single port-in.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortDetail {
    /// Port status word.
    pub status: String,
    /// Internal port ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 5-character port order ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// Account holder name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Account holder email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Firm Order Commitment date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foc: Option<String>,
    /// ISO-8601 creation timestamp.
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// TNs included in this port.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub numbers: Vec<String>,
    /// Carrier message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Response data for `GET /v2.2/inventory`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventorySearchData {
    /// Inventory results.
    pub numbers: Vec<InventoryItem>,
}

/// Response data for `GET /v2.2/inventory/coverage`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryCoverageData {
    /// Aggregated availability buckets.
    pub coverage: Vec<InventoryCoverageItem>,
}

/// One row in [`OrderCreateData::failed`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderFailedEntry {
    /// 10-digit TN that failed.
    pub number: String,
    /// Server-reported reason.
    pub reason: String,
}

/// Response data for `POST /v2.2/orders`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OrderCreateData {
    /// Server-assigned order ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Total amount charged (USD).
    #[serde(rename = "amountCharged")]
    pub amount_charged: f64,
    /// TNs that were successfully ordered.
    #[serde(rename = "numbersOrdered")]
    pub numbers_ordered: Vec<String>,
    /// TNs that failed to order, with reason.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<OrderFailedEntry>,
}

/// Response data for `GET /v2.2/ports`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortListData {
    /// Port-in records on the account.
    pub ports: Vec<PortSummary>,
}

/// Response data for `GET /v2.2/ports/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortDetailData {
    /// Port detail.
    pub port: PortDetail,
}

/// Response data for `POST /v2.2/ports`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortSubmitData {
    /// 5-character port order ID.
    pub pid: String,
    /// Support ticket ID.
    pub ticket: i64,
    /// Carrier message.
    pub message: String,
    /// LOA download URL.
    #[serde(rename = "loaUrl")]
    pub loa_url: String,
    /// Port status URL.
    #[serde(rename = "portUrl")]
    pub port_url: String,
}

/// Response data for `GET /v2.2/ports/availability/{number}` — includes v2.2.10
/// fields `local_routing_number` and `rate_center_tier`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortAvailabilityData {
    /// TN queried.
    pub number: String,
    /// `true` when the TN is portable.
    pub portable: bool,
    /// Losing-carrier service-provider name; `None` when not portable.
    #[serde(rename = "losingCarrier")]
    pub losing_carrier: Option<String>,
    /// LRN of the destination switch (v2.2.10+).
    #[serde(rename = "localRoutingNumber")]
    pub local_routing_number: Option<String>,
    /// Rate-center tier classification (v2.2.10+).
    #[serde(rename = "rateCenterTier")]
    pub rate_center_tier: Option<String>,
    /// Reason; `None` when portable.
    pub reason: Option<String>,
}

/// Query filters for `INumberingService::search_inventory`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InventoryQuery {
    /// 3-digit area code.
    pub npa: Option<i32>,
    /// 3-digit exchange.
    pub nxx: Option<i32>,
    /// Two-letter state code.
    pub state: Option<String>,
    /// Rate-center name.
    pub rate_center: Option<String>,
    /// Substring TNs must contain.
    pub contains: Option<String>,
    /// Suffix TNs must end with.
    pub ends_with: Option<String>,
    /// Max results.
    pub limit: Option<i32>,
}

/// Query filters for `INumberingService::coverage`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoverageQuery {
    /// Two-letter state code.
    pub state: Option<String>,
    /// Rate-center name.
    pub rate_center: Option<String>,
}
