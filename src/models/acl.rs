//! Models for the `ACL` resource.

use serde::{Deserialize, Serialize};

use super::common::CidrEntry;

/// Body for `POST /v2.2/acl` (add) and `DELETE /v2.2/acl` (remove).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclModifyRequest {
    /// CIDR entries to add or remove.
    pub acl: Vec<CidrEntry>,
}

/// Response data for `GET /v2.2/acl`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclListData {
    /// Current allowlist.
    pub acl: Vec<CidrEntry>,
}

/// Response data for `POST /v2.2/acl`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclAddData {
    /// CIDR entries the server actually added.
    pub added: Vec<CidrEntry>,
}

/// Response data for `DELETE /v2.2/acl`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclRemoveData {
    /// CIDR entries the server actually removed.
    pub removed: Vec<CidrEntry>,
}

/// One rejected CIDR with the reason. `reason` is one of:
///
/// - `DB Insert failed`
/// - `DB delete failed`
/// - `Invalid mask: must be /8, /16, /24, or /32`
/// - `CIDR range must be routable`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclFailedEntry {
    /// CIDR that failed.
    pub cidr: String,
    /// Server-reported reason.
    pub reason: String,
}

/// Data payload inside a 409 response from `POST`/`DELETE /v2.2/acl`.
/// Surfaces partial success: the entries that succeeded alongside those that failed.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclConflictData {
    /// CIDR entries added before the conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<CidrEntry>>,
    /// CIDR entries removed before the conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<CidrEntry>>,
    /// CIDR entries the server rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<AclFailedEntry>>,
}
