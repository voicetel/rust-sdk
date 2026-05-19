//! Models for the `Lookups` resource.

use serde::{Deserialize, Serialize};

/// Response data for `GET /v2.2/cnam/{number}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CnamData {
    /// Display name on file, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnam: Option<String>,
    /// TN queried.
    pub number: String,
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
pub struct LrnLookupData {
    /// Echoed ANI.
    pub ani: String,
    /// Echoed destination TN.
    pub destination: String,
    /// LRN dip detail.
    pub lrn: LrnData,
}
