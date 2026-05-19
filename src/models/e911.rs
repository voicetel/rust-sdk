//! Models for the `e911` resource.
//!
//! Note the asymmetric `dn` formats: requests take a 10-digit TN; responses
//! return the 11-digit E.164 US form (country code `1` prepended).

use serde::{Deserialize, Serialize};

/// Body for `POST /v2.2/e911/validations`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911AddressRequest {
    /// Street address line 1.
    pub address1: String,
    /// Street address line 2 (suite/apt/etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// City.
    pub city: String,
    /// Two-letter US state code.
    pub state: String,
    /// 5- or 9-digit ZIP.
    pub zip: String,
}

/// Body for `POST /v2.2/e911` (validate + provision in one call).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911CreateRequest {
    /// 10-digit TN owned by the authenticated account.
    pub dn: String,
    /// Caller name to provision.
    pub callername: String,
    /// Street address line 1.
    pub address1: String,
    /// Street address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// City.
    pub city: String,
    /// Two-letter US state code.
    pub state: String,
    /// 5- or 9-digit ZIP.
    pub zip: String,
}

/// Body for `PUT /v2.2/e911/{dn}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911ProvisionByIdRequest {
    /// Caller name to provision.
    pub callername: String,
    /// Address ID from a prior `POST /v2.2/e911/validations`.
    pub addressid: i64,
}

/// An e911 record bound to a TN.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911Entry {
    /// 11-digit E.164 US TN (leading `1`).
    pub dn: String,
    /// Caller name on record.
    pub callername: String,
    /// Street address line 1.
    pub address1: String,
    /// Street address line 2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// City.
    pub city: String,
    /// State.
    pub state: String,
    /// ZIP.
    pub zip: String,
}

/// Validated address returned by `POST /v2.2/e911/validations`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911ValidatedAddress {
    /// Server-assigned address ID — pass to `Provision`.
    pub addressid: i64,
    /// Address line 1.
    pub address1: String,
    /// Address line 2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// City.
    pub city: String,
    /// State.
    pub state: String,
    /// ZIP.
    pub zip: String,
}

/// Response data for `GET /v2.2/e911`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911AllData {
    /// Every e911 record on the account.
    pub records: Vec<E911Entry>,
}

/// Response data for `GET /v2.2/e911/{dn}`, `POST /v2.2/e911`, and `PUT /v2.2/e911/{dn}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911RecordData {
    /// The record returned.
    pub record: E911Entry,
}

/// Response data for `POST /v2.2/e911/validations`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct E911ValidateData {
    /// Validated address with its assigned ID.
    pub address: E911ValidatedAddress,
}
