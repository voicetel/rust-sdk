//! Models for the `Fax` resource.

use serde::{Deserialize, Serialize};

/// A fax-capable number on the account, as returned by
/// `GET /v2.2/numbers/fax`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaxNumberEntry {
    /// The fax number, ten digits.
    pub number: String,
    /// Per-number fax destination row id, or `0` when faxes deliver to the
    /// account email.
    #[serde(rename = "destinationId")]
    pub destination_id: i64,
    /// Email address faxes to this number deliver to.
    pub email: String,
}

/// Response data for `GET /v2.2/numbers/fax` — every fax-capable number on
/// the account, ascending.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaxNumbersData {
    /// Fax-capable numbers on the account.
    pub numbers: Vec<FaxNumberEntry>,
}

/// One submitted document, as acknowledged by the server.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaxDocumentMeta {
    /// File name.
    pub name: String,
    /// Document size in bytes.
    pub size: i64,
    /// Media type (`application/pdf`, `application/msword`, or the DOCX
    /// office-document type).
    #[serde(rename = "contentType")]
    pub content_type: String,
}

/// Response data for `POST /v2.2/faxes` — the accepted job.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaxSendData {
    /// Identifier of the accepted fax job.
    pub id: String,
    /// The number the fax is sent from, ten digits.
    #[serde(rename = "fromNumber")]
    pub from_number: String,
    /// Destination the fax was accepted for, ten digits.
    #[serde(rename = "toNumber")]
    pub to_number: String,
    /// Job state at acceptance — `queued`.
    pub status: String,
    /// The accepted documents.
    pub documents: Vec<FaxDocumentMeta>,
}

/// Lifecycle state of a fax job.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FaxJobStatus {
    /// Accepted, waiting for the processor to pick it up.
    #[default]
    Queued,
    /// Delivery in progress.
    Sending,
    /// A delivery attempt failed and will be retried.
    RetryPending,
    /// Delivered.
    Sent,
    /// Terminally failed — see `failure_reason`.
    Failed,
}

/// One fax job: routing, lifecycle timestamps, attempts and documents.
///
/// Returned by `GET /v2.2/faxes/{id}` and inside [`FaxHistoryData`].
/// Timestamps are ISO-8601 UTC strings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaxJob {
    /// Fax job identifier.
    pub id: String,
    /// The number the fax was sent from, ten digits.
    #[serde(rename = "fromNumber")]
    pub from_number: String,
    /// The destination, ten digits.
    #[serde(rename = "toNumber")]
    pub to_number: String,
    /// The `Idempotency-Key` the fax was submitted with, or `None`.
    #[serde(rename = "clientToken", skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// Job state.
    pub status: FaxJobStatus,
    /// Acceptance time, ISO-8601 UTC.
    #[serde(rename = "submittedAt")]
    pub submitted_at: String,
    /// When delivery was first attempted, or `None` while queued.
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// When the job reached `Sent` or `Failed`, or `None` while active.
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Delivery attempts so far, or `None` while queued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    /// Why the fax failed, on `Failed` jobs; `None` otherwise.
    #[serde(rename = "failureReason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The submitted documents.
    pub documents: Vec<FaxDocumentMeta>,
    /// Whether `FaxService::document` can serve the PDF for this job.
    #[serde(rename = "documentAvailable")]
    pub document_available: bool,
    /// File name the document downloads as, when a document is kept.
    #[serde(rename = "documentName", skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// Total document size in bytes, or `None` when there are no documents.
    #[serde(rename = "documentSize", skip_serializing_if = "Option::is_none")]
    pub document_size: Option<i64>,
}

/// Response data for `GET /v2.2/faxes` — one page of history, newest first.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaxHistoryData {
    /// The number the history belongs to, ten digits.
    pub number: String,
    /// Newer edge of the returned range, unix seconds.
    #[serde(rename = "fromTs")]
    pub from_ts: i64,
    /// Older edge of the returned range, unix seconds.
    #[serde(rename = "toTs")]
    pub to_ts: i64,
    /// Fax jobs, newest first.
    pub faxes: Vec<FaxJob>,
    /// Cursor for the next page, or `None` when the page is the last.
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Status filter for `GET /v2.2/faxes`; `Active` means "not yet finished".
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FaxHistoryStatus {
    /// Every job state.
    #[default]
    All,
    /// Not yet finished.
    Active,
    /// Accepted, waiting for the processor.
    Queued,
    /// Delivery in progress.
    Sending,
    /// A delivery attempt failed and will be retried.
    RetryPending,
    /// Delivered.
    Sent,
    /// Terminally failed.
    Failed,
}

impl FaxHistoryStatus {
    /// Wire value of the `status` query parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Active => "active",
            Self::Queued => "queued",
            Self::Sending => "sending",
            Self::RetryPending => "retry_pending",
            Self::Sent => "sent",
            Self::Failed => "failed",
        }
    }
}

/// Options for `FaxService::history` (`GET /v2.2/faxes`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FaxHistoryOptions {
    /// A fax number on the account whose history to read (required).
    pub number: String,
    /// Newer edge of the range, unix seconds. Server default: now.
    pub from: Option<i64>,
    /// Older edge of the range, unix seconds. Server default: 24 hours ago.
    pub to: Option<i64>,
    /// Page size, 1 to 100. Server default: 50.
    pub limit: Option<u32>,
    /// Paging cursor from a previous response's `next_cursor`.
    pub cursor: Option<String>,
    /// Filter by job state; `Active` means not yet finished.
    pub status: Option<FaxHistoryStatus>,
}

/// One document to fax: raw bytes plus name and MIME type.
///
/// The server checks content (magic bytes), not the extension — the
/// `content_type` must be `application/pdf`, `application/msword`, or the
/// DOCX office-document type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaxDocument {
    /// File name for the document part.
    pub name: String,
    /// Document MIME type.
    pub content_type: String,
    /// Raw document bytes — at most 8 MiB per document and 8 MiB total.
    pub data: Vec<u8>,
}

/// Request for `FaxService::send` (`POST /v2.2/faxes`).
///
/// Travels as `multipart/form-data`, not JSON: `fromNumber` and `toNumber`
/// are text fields and each document is a file part. `idempotency_key`
/// becomes the required `Idempotency-Key` header — a fresh UUID is generated
/// when left as `None`; resubmitting the same documents with the same key
/// returns the original job instead of sending twice.
#[derive(Debug, Clone, Default)]
pub struct FaxSendRequest {
    /// Fax number on the account to send from, ten digits.
    pub from_number: String,
    /// Destination US or Canadian fax number, ten digits.
    pub to_number: String,
    /// One to ten documents, at most 8 MiB each and 8 MiB in total.
    pub documents: Vec<FaxDocument>,
    /// Client-generated UUID deduplicating this submission. When `None`, a
    /// fresh UUID is generated for the `Idempotency-Key` header.
    pub idempotency_key: Option<String>,
}
