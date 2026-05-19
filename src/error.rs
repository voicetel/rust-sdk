//! Error types for the VoiceTel SDK.
//!
//! Every fallible method returns `Result<T, ApiError>`. `ApiError` carries
//! a typed [`ErrorKind`] so callers can `match` on the failure mode without
//! pattern-matching on HTTP status codes.

use std::fmt;

/// Classifies a VoiceTel API failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Catch-all for unmapped statuses and transport-level failures.
    Unknown,
    /// HTTP 400 — server-side validation failed.
    BadRequest,
    /// HTTP 401 — bearer token missing, expired, or invalid.
    Authentication,
    /// HTTP 403 — authenticated but not authorised for the resource.
    PermissionDenied,
    /// HTTP 404 — resource does not exist.
    NotFound,
    /// HTTP 409 — request conflicts with current state.
    Conflict,
    /// HTTP 429 — rate-limit exceeded.
    RateLimit,
    /// Any HTTP 5xx response.
    Server,
    /// Local transport error (DNS, TLS, connection, timeout) before the
    /// server saw the request.
    Transport,
    /// Failed to (de)serialize a request or response body.
    Serialization,
}

impl ErrorKind {
    /// Map an HTTP status code to its corresponding `ErrorKind`.
    pub fn from_status(status: u16) -> Self {
        match status {
            400 => Self::BadRequest,
            401 => Self::Authentication,
            403 => Self::PermissionDenied,
            404 => Self::NotFound,
            409 => Self::Conflict,
            429 => Self::RateLimit,
            500..=599 => Self::Server,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::BadRequest => "bad request",
            Self::Authentication => "authentication",
            Self::PermissionDenied => "permission denied",
            Self::NotFound => "not found",
            Self::Conflict => "conflict",
            Self::RateLimit => "rate limit",
            Self::Server => "server error",
            Self::Transport => "transport error",
            Self::Serialization => "serialization error",
        })
    }
}

/// The error returned by every fallible SDK call.
///
/// Inspect `kind` to branch on the failure mode. `body` preserves the raw
/// response text — particularly useful on 409 conflicts where the server
/// returns a structured detail block (see `AclConflictData`).
#[derive(Debug)]
pub struct ApiError {
    /// Typed classification of the failure.
    pub kind: ErrorKind,
    /// HTTP status code, or `0` for transport-level failures.
    pub status_code: u16,
    /// `code` / `error` field extracted from the JSON error body when present.
    pub code: Option<String>,
    /// Human-readable message describing the failure.
    pub message: String,
    /// Raw response body as a string, when one was received.
    pub body: Option<String>,
    /// Underlying transport/serialization error, when applicable.
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ApiError {
    /// Construct an API error from an HTTP status code, optional error code,
    /// message, and raw response body.
    pub fn from_status(
        status: u16,
        code: Option<String>,
        message: impl Into<String>,
        body: Option<String>,
    ) -> Self {
        Self {
            kind: ErrorKind::from_status(status),
            status_code: status,
            code,
            message: message.into(),
            body,
            source: None,
        }
    }

    /// Build an error of the given kind with a message and no underlying source.
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            status_code: 0,
            code: None,
            message: message.into(),
            body: None,
            source: None,
        }
    }

    /// Build a transport-level error wrapping the underlying cause.
    pub fn transport(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Transport,
            status_code: 0,
            code: None,
            message: message.into(),
            body: None,
            source: Some(Box::new(source)),
        }
    }

    /// Build a serialization-level error wrapping the underlying cause.
    pub fn serialization(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Serialization,
            status_code: 0,
            code: None,
            message: message.into(),
            body: None,
            source: Some(Box::new(source)),
        }
    }

    /// `true` when [`Self::kind`] is [`ErrorKind::RateLimit`].
    pub fn is_rate_limit(&self) -> bool {
        self.kind == ErrorKind::RateLimit
    }

    /// `true` when [`Self::kind`] is [`ErrorKind::NotFound`].
    pub fn is_not_found(&self) -> bool {
        self.kind == ErrorKind::NotFound
    }

    /// `true` when [`Self::kind`] is [`ErrorKind::Authentication`].
    pub fn is_authentication(&self) -> bool {
        self.kind == ErrorKind::Authentication
    }

    /// `true` when [`Self::kind`] is [`ErrorKind::Conflict`].
    pub fn is_conflict(&self) -> bool {
        self.kind == ErrorKind::Conflict
    }

    /// Attach (or clear) the underlying source error. `pub(crate)` because
    /// only the transport layer needs to chain causes onto a prebuilt error.
    pub(crate) fn set_source_internal(
        &mut self,
        src: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) {
        self.source = src;
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.status_code != 0 {
            if let Some(code) = &self.code {
                write!(
                    f,
                    "voicetel: HTTP {} {}: {}",
                    self.status_code, code, self.message
                )
            } else {
                write!(f, "voicetel: HTTP {}: {}", self.status_code, self.message)
            }
        } else {
            write!(f, "voicetel: {}: {}", self.kind, self.message)
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| &**s as &(dyn std::error::Error + 'static))
    }
}
