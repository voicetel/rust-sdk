//! Models for the `Lookups` resource.

use serde::{Deserialize, Serialize};

/// Response data for `GET /v2.2/cnam/{number}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CnamData {
    /// Display name on file, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnam: Option<String>,
    /// TN queried.
    pub number: String,
}

/// Carrier and line-type details returned by a carrier lookup.
///
/// Field names are the upstream carrier lookup's own, so the wire is
/// snake_case (`mobile_country_code`, `mobile_network_code`, `error_code`) —
/// exactly as in Rust.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CarrierInfo {
    /// Carrier or network name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Line type, such as `voip`, `landline`, or `mobile`.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Mobile country code, present on mobile numbers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_country_code: Option<String>,
    /// Mobile network code, present on mobile numbers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_network_code: Option<String>,
    /// Error code when the carrier details could not be determined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
}

/// Response data for `GET /v2.2/carrier/{number}`.
///
/// Billed per lookup at the account's CNAM rate, only when a result comes
/// back.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CarrierData {
    /// TN queried.
    pub number: String,
    /// Carrier and line-type details for the number.
    pub carrier: CarrierInfo,
}

/// LRN dip result.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LrnData {
    /// Local Routing Number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lrn: Option<String>,
    /// State.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Rate center.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rc: Option<String>,
    /// LATA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lata: Option<String>,
    /// Operating Carrier Number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocn: Option<String>,
    /// Local Exchange Carrier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lec: Option<String>,
    /// LEC classification.
    #[serde(default, rename = "lecType", skip_serializing_if = "Option::is_none")]
    pub lec_type: Option<String>,
    /// Jurisdiction code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// `Y`/`N` — local to the ANI's rate center.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<String>,
}

/// Response data for `GET /v2.2/lrn/{number}/{ani}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LrnLookupData {
    /// Echoed ANI.
    pub ani: String,
    /// Echoed destination TN.
    pub destination: String,
    /// LRN dip detail — `None` when the dip found no record (still 200,
    /// still billed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lrn: Option<LrnData>,
}
