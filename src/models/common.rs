//! Cross-resource shared types.

use serde::{Deserialize, Serialize};

/// One row in the IP allowlist.
///
/// `mask` must be `/8`, `/16`, `/24`, or `/32` and must describe a routable
/// public address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CidrEntry {
    /// CIDR-notation IPv4 block (e.g. `203.0.113.0/24`).
    pub cidr: String,
}
