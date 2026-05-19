//! Models for the `Authentication` resource.

use serde::{Deserialize, Serialize};

use super::common::CidrEntry;

/// Auth-mode constants for [`AuthPutRequest::auth_type`] /
/// [`AuthGetData::auth_type`].
pub mod auth_type {
    /// SIP Digest authentication only.
    pub const DIGEST: i32 = 0;
    /// IP allowlist only.
    pub const IP_AUTH: i32 = 1;
    /// Digest OR IP allowlist.
    pub const DIGEST_OR_IP: i32 = 2;
    /// Digest AND IP allowlist.
    pub const DIGEST_AND_IP: i32 = 3;
}

/// Body for `PUT /v2.2/auth`. All fields optional.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthPutRequest {
    /// Auth mode (see [`auth_type`]).
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<i32>,
    /// New password (6-10 alphanumeric; at least one letter and one digit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Response data for `GET /v2.2/auth`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthGetData {
    /// Currently-active auth mode.
    #[serde(rename = "authType")]
    pub auth_type: i32,
    /// Human-readable description of the auth mode.
    #[serde(rename = "authTypeDescription")]
    pub auth_type_description: String,
    /// Current IP allowlist.
    pub acl: Vec<CidrEntry>,
}

/// One field's change, returned by `PUT /v2.2/auth`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthUpdatedEntry {
    /// `authType` or `password`.
    pub field: String,
    /// Echoed value (only present when echoing is safe — `authType`, never `password`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

/// Response data for `PUT /v2.2/auth`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthPutData {
    /// Fields the server applied.
    pub updated: Vec<AuthUpdatedEntry>,
}

/// Data payload returned in a 409 from `PUT /v2.2/auth`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthPutConflictData {
    /// Fields the server applied before the conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<AuthUpdatedEntry>>,
}
