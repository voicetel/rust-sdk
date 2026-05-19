//! Version constants for the VoiceTel Rust SDK.

/// This SDK crate's semantic version.
pub const SDK_VERSION: &str = "2.2.10";

/// The VoiceTel REST API version targeted by this SDK.
pub const API_VERSION: &str = "v2.2.10";

/// Production VoiceTel API endpoint.
pub const DEFAULT_BASE_URL: &str = "https://api.voicetel.com";

/// Default `User-Agent` sent on every request.
pub const DEFAULT_USER_AGENT: &str = concat!(
    "voicetel-rust/",
    "2.2.10",
    " (+https://github.com/voicetel/rust-sdk)"
);
