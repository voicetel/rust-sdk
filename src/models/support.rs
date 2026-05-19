//! Models for the `Support` resource.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Body for `POST /v2.2/support/tickets`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketCreateRequest {
    /// Ticket subject.
    pub subject: String,
    /// Initial message body.
    pub message: String,
    /// Admin-only: create on behalf of this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Body for `PUT /v2.2/support/tickets/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketUpdateRequest {
    /// One of `active`, `pending`, `closed`, `spam`.
    pub status: String,
}

/// Body for `POST /v2.2/support/tickets/{id}/replies`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketReplyRequest {
    /// Reply body.
    pub message: String,
}

/// `source` shape on a ticket or thread.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketSource {
    /// Channel — `email`, `web`, …
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
    /// Source type.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// `action` descriptor on a thread.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketAction {
    /// Action text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Action type.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// Actor (createdBy / assignee / assignedTo / closedByUser).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketActor {
    /// Actor ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// `customer` or `user`.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// First name.
    #[serde(default, rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(default, rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Photo URL.
    #[serde(default, rename = "photoUrl", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

/// One custom-field row on a conversation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomFieldValue {
    /// Field ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Underlying value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Display text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// `{id, value, type}` entry under `embedded.emails` / `phones` / `socialProfiles`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomerContactEntry {
    /// Row ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Underlying value (email / phone / handle).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Entry type (e.g. `work`, `mobile`).
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// `{id, value}` entry under `embedded.websites`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomerWebsiteEntry {
    /// Row ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `embedded.address` on a [`SupportCustomer`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomerAddress {
    /// Street.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter US state code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 5- or 9-digit ZIP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

/// Embedded shape on a [`SupportCustomer`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomerEmbedded {
    /// Address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<CustomerAddress>,
    /// Emails.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<CustomerContactEntry>,
    /// Phones.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<CustomerContactEntry>,
    /// Social profiles.
    #[serde(
        default,
        rename = "socialProfiles",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub social_profiles: Vec<CustomerContactEntry>,
    /// Websites.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websites: Vec<CustomerWebsiteEntry>,
}

/// One file attached to a support thread.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportAttachment {
    /// Attachment ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// MIME type.
    #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Original filename.
    #[serde(default, rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Download URL.
    #[serde(default, rename = "fileUrl", skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// Size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Embedded shape on a [`SupportThread`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreadEmbedded {
    /// Attachments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<SupportAttachment>,
}

/// Embedded shape on a [`SupportConversation`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversationEmbedded {
    /// Threads.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threads: Vec<SupportThread>,
}

/// End-user profile attached to a support ticket.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportCustomer {
    /// Customer ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// First name.
    #[serde(default, rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(default, rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Company.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Job title.
    #[serde(default, rename = "jobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// Photo type.
    #[serde(default, rename = "photoType", skip_serializing_if = "Option::is_none")]
    pub photo_type: Option<String>,
    /// Photo URL.
    #[serde(default, rename = "photoUrl", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Always `customer`.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Creation timestamp.
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last update timestamp.
    #[serde(default, rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Embedded contact info.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedded: Option<CustomerEmbedded>,
}

/// One message in a ticket conversation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportThread {
    /// Thread ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Status word.
    pub status: String,
    /// State word.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// `customer`, `message`, or `note`.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Thread body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Rating value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    /// Rating comment.
    #[serde(
        default,
        rename = "ratingComment",
        skip_serializing_if = "Option::is_none"
    )]
    pub rating_comment: Option<String>,
    /// Open timestamp.
    #[serde(default, rename = "openedAt", skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
    /// Creation timestamp.
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Source channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<TicketSource>,
    /// Action descriptor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<TicketAction>,
    /// Author.
    #[serde(default, rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TicketActor>,
    /// Assignee.
    #[serde(
        default,
        rename = "assignedTo",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<TicketActor>,
    /// Customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<SupportCustomer>,
    /// `To` recipients.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<String>,
    /// `Cc` recipients.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<String>,
    /// `Bcc` recipients.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<String>,
    /// Embedded attachments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedded: Option<ThreadEmbedded>,
}

/// A support ticket.
///
/// **Note:** the wire field `number` is a ticket sequence integer (1015, 2114,
/// ...), **not** a phone number. The Rust SDK exposes it as
/// `ticket_number: Option<i64>` to avoid confusion with 10-digit TNs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SupportConversation {
    /// Ticket ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Human-readable ticket sequence number (wire field `number`).
    #[serde(default, rename = "number", skip_serializing_if = "Option::is_none")]
    pub ticket_number: Option<i64>,
    /// Status word.
    pub status: String,
    /// State word.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Subject line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Preview blurb.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    /// Conversation type.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Mailbox ID.
    #[serde(default, rename = "mailboxId", skip_serializing_if = "Option::is_none")]
    pub mailbox_id: Option<i64>,
    /// Folder ID.
    #[serde(default, rename = "folderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    /// Thread count.
    #[serde(
        default,
        rename = "threadsCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub threads_count: Option<i64>,
    /// Closed-by actor ID.
    #[serde(default, rename = "closedBy", skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<i64>,
    /// Close timestamp.
    #[serde(default, rename = "closedAt", skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    /// Creation timestamp.
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Update timestamp.
    #[serde(default, rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// User-update timestamp.
    #[serde(
        default,
        rename = "userUpdatedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_updated_at: Option<String>,
    /// Customer-waiting-since metadata.
    #[serde(
        default,
        rename = "customerWaitingSince",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_waiting_since: Option<Value>,
    /// Source channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<TicketSource>,
    /// Author.
    #[serde(default, rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TicketActor>,
    /// Assignee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<TicketActor>,
    /// Actor that closed the ticket.
    #[serde(
        default,
        rename = "closedByUser",
        skip_serializing_if = "Option::is_none"
    )]
    pub closed_by_user: Option<TicketActor>,
    /// Associated customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<SupportCustomer>,
    /// `Cc` recipients.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<String>,
    /// `Bcc` recipients.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<String>,
    /// Custom fields.
    #[serde(
        default,
        rename = "customFields",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_fields: Vec<CustomFieldValue>,
    /// Embedded threads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedded: Option<ConversationEmbedded>,
}

/// Response data for `GET`/`POST /v2.2/support/tickets/{...}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TicketData {
    /// The ticket.
    pub ticket: SupportConversation,
}

/// Response data for `GET /v2.2/support/tickets`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TicketsListData {
    /// Every ticket on the account.
    pub tickets: Vec<SupportConversation>,
}

/// Response data for `GET /v2.2/support/tickets/{id}/messages`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketThreadsData {
    /// Threads.
    pub messages: Vec<SupportThread>,
}

/// Response data for `POST /v2.2/support/tickets/{id}/replies`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketReplyData {
    /// Always `Reply added`.
    pub message: String,
}

/// Response data for `PUT /v2.2/support/tickets/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketUpdateData {
    /// Ticket ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Outcome word, e.g. `success`.
    pub status: String,
}
