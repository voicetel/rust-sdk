//! `Fax` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::fax::*;
use crate::transport::{encode_path_segment, FormPart};

/// Handle for the `Fax` resource — send faxes from fax-capable numbers and
/// read their history and documents.
#[derive(Debug)]
pub struct FaxService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> FaxService<'c> {
    /// `GET /v2.2/numbers/fax` — every fax-capable number on the account,
    /// with the email address each delivers to (`destination_id` `0` means
    /// the account email).
    pub async fn numbers(&self) -> Result<FaxNumbersData, ApiError> {
        self.client
            .transport
            .request::<(), FaxNumbersData>(Method::GET, "/v2.2/numbers/fax", &[], None, true)
            .await
    }

    /// `POST /v2.2/faxes` — send a fax.
    ///
    /// The request goes out as `multipart/form-data` with a client-generated
    /// UUID in the required `Idempotency-Key` header; resubmitting the same
    /// documents with the same key returns the original job instead of
    /// sending twice. Rate-limited to 10 sends per minute per account.
    /// Errors carry the standard envelope (plus a `serverTime` field, which
    /// is ignored).
    pub async fn send(&self, body: &FaxSendRequest) -> Result<FaxSendData, ApiError> {
        let mut parts: Vec<FormPart> = Vec::with_capacity(2 + body.documents.len());
        parts.push(FormPart::field("fromNumber", &body.from_number));
        parts.push(FormPart::field("toNumber", &body.to_number));
        for doc in &body.documents {
            parts.push(FormPart::file(
                "documents",
                &doc.name,
                &doc.content_type,
                &doc.data,
            ));
        }
        self.client
            .transport
            .request_multipart(
                Method::POST,
                "/v2.2/faxes",
                &parts,
                true,
                body.idempotency_key.as_deref(),
            )
            .await
    }

    /// `GET /v2.2/faxes` — the faxes sent from one of your fax numbers,
    /// newest first, with cursor paging. `from`/`to` are unix seconds
    /// (`from` is the newer edge); the server defaults to the last 24 hours.
    pub async fn history(&self, opts: &FaxHistoryOptions) -> Result<FaxHistoryData, ApiError> {
        let mut q: Vec<(&str, String)> = vec![("number", opts.number.clone())];
        if let Some(f) = opts.from {
            q.push(("from", f.to_string()));
        }
        if let Some(t) = opts.to {
            q.push(("to", t.to_string()));
        }
        if let Some(l) = opts.limit {
            q.push(("limit", l.to_string()));
        }
        if let Some(ref cur) = opts.cursor {
            q.push(("cursor", cur.clone()));
        }
        if let Some(s) = opts.status {
            q.push(("status", s.as_str().to_string()));
        }
        self.client
            .transport
            .request::<(), FaxHistoryData>(Method::GET, "/v2.2/faxes", &q, None, true)
            .await
    }

    /// `GET /v2.2/faxes/{id}` — one job with its status, timestamps,
    /// attempts and documents. Poll until `status` is `Sent` or `Failed`.
    pub async fn get(&self, id: &str) -> Result<FaxJob, ApiError> {
        let path = format!("/v2.2/faxes/{}", encode_path_segment(id));
        self.client
            .transport
            .request::<(), FaxJob>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `GET /v2.2/faxes/{id}/document` — the PDF rendering of the fax as
    /// raw bytes (served when the job reports `document_available`).
    pub async fn document(&self, id: &str) -> Result<Vec<u8>, ApiError> {
        let path = format!("/v2.2/faxes/{}/document", encode_path_segment(id));
        self.client
            .transport
            .request_bytes(Method::GET, &path, &[], true)
            .await
    }
}
