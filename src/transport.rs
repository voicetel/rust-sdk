//! Low-level HTTP transport: request building, retry, and envelope unwrap.
//!
//! The [`Transport`] is held inside [`crate::Client`] and is not constructed
//! directly by SDK users.

use std::sync::{Arc, RwLock};
use std::time::Duration;

use reqwest::{header, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use uuid::Uuid;

use crate::error::{ApiError, ErrorKind};

/// Retryable HTTP status codes (429 and selected 5xx).
const RETRYABLE_STATUSES: &[u16] = &[429, 500, 502, 503, 504];

/// One part of a `multipart/form-data` request body.
///
/// Built with [`FormPart::field`] (plain text field) or [`FormPart::file`]
/// (file part with a name and MIME type). Passed to
/// [`Transport::request_multipart`].
#[derive(Debug, Clone)]
pub struct FormPart {
    /// Form field name.
    name: String,
    /// File name — `Some` for file parts, `None` for plain fields.
    filename: Option<String>,
    /// Part MIME type. Sent as the part's `Content-Type` header on file
    /// parts; ignored on plain fields.
    content_type: Option<String>,
    /// Part payload.
    data: Vec<u8>,
}

impl FormPart {
    /// A plain text form field.
    pub fn field(name: impl Into<String>, value: impl AsRef<str>) -> Self {
        Self {
            name: name.into(),
            filename: None,
            content_type: None,
            data: value.as_ref().as_bytes().to_vec(),
        }
    }

    /// A file part carrying raw bytes, a file name, and a MIME type.
    pub fn file(
        name: impl Into<String>,
        filename: impl Into<String>,
        content_type: impl Into<String>,
        data: impl AsRef<[u8]>,
    ) -> Self {
        Self {
            name: name.into(),
            filename: Some(filename.into()),
            content_type: Some(content_type.into()),
            data: data.as_ref().to_vec(),
        }
    }
}

/// Internal HTTP client used by every resource service.
#[derive(Debug, Clone)]
pub struct Transport {
    pub(crate) base_url: String,
    pub(crate) user_agent: String,
    pub(crate) max_retries: u32,
    pub(crate) http: reqwest::Client,
    pub(crate) api_key: Arc<RwLock<Option<String>>>,
}

impl Transport {
    /// Read the currently-installed bearer token.
    pub fn api_key(&self) -> Option<String> {
        self.api_key.read().ok().and_then(|g| g.clone())
    }

    /// Install (or clear) the bearer token used on every authenticated request.
    pub fn set_api_key(&self, key: Option<String>) {
        if let Ok(mut g) = self.api_key.write() {
            *g = key;
        }
    }

    /// Perform a JSON-in / JSON-out request, retrying transient failures.
    ///
    /// `require_auth=false` skips the `Authorization` header — used by the
    /// public endpoints: the api-key exchange, account signup, and the
    /// password-recovery endpoint.
    pub async fn request<B, T>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&B>,
        require_auth: bool,
    ) -> Result<T, ApiError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let raw = self
            .request_raw(method, path, query, body, require_auth)
            .await?;
        decode_envelope(&raw)
    }

    /// Perform a `multipart/form-data` request with a JSON-envelope response —
    /// `POST /v2.2/faxes` (fax send) is the one such endpoint.
    ///
    /// `idempotency_key` overrides the `Idempotency-Key` header; when `None`
    /// a fresh UUID is generated for POST/PUT/PATCH requests, exactly as on
    /// the JSON path.
    pub async fn request_multipart<T>(
        &self,
        method: Method,
        path: &str,
        parts: &[FormPart],
        require_auth: bool,
        idempotency_key: Option<&str>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        self.ensure_auth(require_auth)?;
        let boundary = format!("voicetel-{}", Uuid::new_v4().simple());
        let body = build_multipart_body(parts, &boundary);
        let content_type = Some(format!("multipart/form-data; boundary={}", boundary));
        let idempotency_key = match idempotency_key {
            Some(key) => Some(key.to_string()),
            None if matches!(method, Method::POST | Method::PUT | Method::PATCH) => {
                Some(Uuid::new_v4().to_string())
            }
            None => None,
        };
        let url = format!("{}{}", self.base_url, path);
        let raw = self
            .send_loop(
                method,
                &url,
                Some(body),
                content_type.as_deref(),
                idempotency_key,
                require_auth,
            )
            .await?;
        decode_envelope(&raw)
    }

    /// Perform a request whose successful response is a binary payload rather
    /// than the JSON envelope — `GET /v2.2/faxes/{id}/document` (the PDF
    /// download) is the one such endpoint. Returns the raw body bytes.
    pub async fn request_bytes(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        require_auth: bool,
    ) -> Result<Vec<u8>, ApiError> {
        self.request_raw::<()>(method, path, query, None, require_auth)
            .await
    }

    /// Perform a request whose successful response carries no body
    /// (204 No Content). Returns `Ok(())` on success.
    pub async fn request_no_content<B>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&B>,
        require_auth: bool,
    ) -> Result<(), ApiError>
    where
        B: Serialize + ?Sized,
    {
        let _ = self
            .request_raw(method, path, query, body, require_auth)
            .await?;
        Ok(())
    }

    /// Inner request loop for the JSON path. Returns the raw response body
    /// bytes on 2xx, or an `ApiError` for any other outcome.
    async fn request_raw<B>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&B>,
        require_auth: bool,
    ) -> Result<Vec<u8>, ApiError>
    where
        B: Serialize + ?Sized,
    {
        self.ensure_auth(require_auth)?;

        let mut url = format!("{}{}", self.base_url, path);
        if !query.is_empty() {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            for (k, v) in query {
                serializer.append_pair(k, v);
            }
            url.push('?');
            url.push_str(&serializer.finish());
        }

        let body_bytes = if let Some(b) = body {
            Some(
                serde_json::to_vec(b)
                    .map_err(|e| ApiError::serialization("marshal request body", e))?,
            )
        } else {
            None
        };

        let idempotency_key = matches!(method, Method::POST | Method::PUT | Method::PATCH)
            .then(|| Uuid::new_v4().to_string());

        self.send_loop(
            method,
            &url,
            body_bytes,
            body.map(|_| "application/json"),
            idempotency_key,
            require_auth,
        )
        .await
    }

    /// Shared request loop: send, retry transient failures (429 / 5xx with
    /// exponential backoff), and return the raw body bytes on 2xx or an
    /// `ApiError` otherwise.
    async fn send_loop(
        &self,
        method: Method,
        url: &str,
        body: Option<Vec<u8>>,
        content_type: Option<&str>,
        idempotency_key: Option<String>,
        require_auth: bool,
    ) -> Result<Vec<u8>, ApiError> {
        let mut attempt: u32 = 0;
        loop {
            let mut req = self
                .http
                .request(method.clone(), url)
                .header(header::USER_AGENT, &self.user_agent)
                .header(header::ACCEPT, "application/json")
                .header(header::ACCEPT_ENCODING, "gzip");
            if let Some(ref bytes) = body {
                req = req
                    .header(
                        header::CONTENT_TYPE,
                        content_type.unwrap_or("application/json"),
                    )
                    .body(bytes.clone());
            }
            if require_auth {
                if let Some(key) = self.api_key() {
                    req = req.header(header::AUTHORIZATION, format!("Bearer {}", key));
                }
            }
            if let Some(ref key) = idempotency_key {
                req = req.header("Idempotency-Key", key.as_str());
            }

            let resp = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    if attempt >= self.max_retries {
                        return Err(ApiError::transport(
                            format!("transport error after {} attempt(s)", attempt + 1),
                            e,
                        ));
                    }
                    let delay = backoff_delay(attempt, None);
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    continue;
                }
            };

            let status = resp.status();
            let retry_after = resp
                .headers()
                .get(header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            if RETRYABLE_STATUSES.contains(&status.as_u16()) && attempt < self.max_retries {
                let delay = backoff_delay(attempt, retry_after);
                // Drain body so the connection can be reused.
                drop(resp.bytes().await.ok());
                tokio::time::sleep(delay).await;
                attempt += 1;
                continue;
            }

            let bytes = resp
                .bytes()
                .await
                .map_err(|e| ApiError::transport("read response body", e))?
                .to_vec();
            return decode(status, bytes);
        }
    }

    /// Reject an authenticated call before any request goes out when no API
    /// key is installed.
    fn ensure_auth(&self, require_auth: bool) -> Result<(), ApiError> {
        if require_auth && self.api_key().is_none() {
            return Err(ApiError::new(
                ErrorKind::Authentication,
                "no api key set; call Client::login or use Client::builder().api_key(...)",
            ));
        }
        Ok(())
    }
}

/// Decode a successful JSON-envelope response: reject an empty body, strip
/// the `{status, data}` wrapper, and deserialize the inner payload as `T`.
fn decode_envelope<T: DeserializeOwned>(raw: &[u8]) -> Result<T, ApiError> {
    // 204 No Content / empty body for methods returning `()` — handled
    // by the dedicated `request_no_content` path. Anything else with an
    // empty body is an error here.
    if raw.is_empty() {
        return Err(ApiError::new(
            ErrorKind::Serialization,
            "empty response body when one was expected",
        ));
    }
    let inner = unwrap_envelope(raw)?;
    serde_json::from_slice::<T>(&inner).map_err(|e| {
        let mut err = ApiError::new(ErrorKind::Serialization, "decode response body");
        err.body = Some(String::from_utf8_lossy(&inner).into_owned());
        err.set_source_internal(Some(Box::new(e)));
        err
    })
}

/// Serialize the parts into a `multipart/form-data` body delimited by
/// `boundary`, following RFC 7578: each part carries a
/// `Content-Disposition: form-data` header (with `filename` and a part
/// `Content-Type` on file parts), and the body ends with a closing
/// `--boundary--` delimiter.
pub(crate) fn build_multipart_body(parts: &[FormPart], boundary: &str) -> Vec<u8> {
    let mut body = Vec::new();
    for part in parts {
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        match &part.filename {
            Some(filename) => {
                body.extend_from_slice(
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        sanitize_part_header(&part.name),
                        sanitize_part_header(filename)
                    )
                    .as_bytes(),
                );
                body.extend_from_slice(
                    format!(
                        "Content-Type: {}\r\n",
                        part.content_type
                            .as_deref()
                            .unwrap_or("application/octet-stream")
                    )
                    .as_bytes(),
                );
            }
            None => {
                body.extend_from_slice(
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"\r\n",
                        sanitize_part_header(&part.name)
                    )
                    .as_bytes(),
                );
            }
        }
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(&part.data);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
    body
}

/// Make a `Content-Disposition` name or filename safe to embed in a quoted
/// string: drop CR/LF (header folding) and neutralise quotes/backslashes.
fn sanitize_part_header(value: &str) -> String {
    value
        .replace(['\r', '\n'], "")
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

/// Inspect a final response: return bytes on 2xx; build an `ApiError` otherwise.
///
/// The spec error envelope is `{"status":"error","error":{"code":<int>,
/// "message":<string>}}` — the nested object is parsed first. A legacy flat
/// `{"code": ..., "message": ...}` shape is still tolerated as a fallback.
fn decode(status: StatusCode, body: Vec<u8>) -> Result<Vec<u8>, ApiError> {
    if status.is_success() {
        return Ok(body);
    }
    let text = String::from_utf8_lossy(&body).into_owned();
    let mut code = None;
    let mut message = format!("HTTP {}", status.as_u16());
    if let Ok(val) = serde_json::from_slice::<Value>(&body) {
        if let Some(obj) = val.as_object() {
            if let Some(err) = obj.get("error").and_then(|v| v.as_object()) {
                if let Some(c) = err.get("code").and_then(|v| v.as_i64()) {
                    code = Some(c);
                }
                if let Some(m) = err.get("message").and_then(|v| v.as_str()) {
                    message = m.to_string();
                }
            } else {
                if let Some(c) = obj.get("code").and_then(|v| v.as_i64()) {
                    code = Some(c);
                }
                if let Some(m) = obj.get("message").and_then(|v| v.as_str()) {
                    message = m.to_string();
                } else if let Some(m) = obj.get("error").and_then(|v| v.as_str()) {
                    message = m.to_string();
                }
            }
        }
    }
    Err(ApiError::from_status(
        status.as_u16(),
        code,
        message,
        if text.is_empty() { None } else { Some(text) },
    ))
}

/// Strip the `{"status": "...", "data": ...}` envelope when present.
/// Returns the inner `data` payload, or the input unchanged when no envelope.
pub fn unwrap_envelope(raw: &[u8]) -> Result<Vec<u8>, ApiError> {
    let trimmed = trim_leading_whitespace(raw);
    if trimmed.is_empty() || trimmed[0] != b'{' {
        return Ok(raw.to_vec());
    }
    let value: Value = match serde_json::from_slice(raw) {
        Ok(v) => v,
        Err(e) => {
            let mut err = ApiError::new(ErrorKind::Serialization, "decode response envelope");
            err.body = Some(String::from_utf8_lossy(raw).into_owned());
            err.set_source_internal(Some(Box::new(e)));
            return Err(err);
        }
    };
    if let Value::Object(map) = &value {
        if map.contains_key("status") {
            if let Some(data) = map.get("data") {
                return serde_json::to_vec(data).map_err(|e| {
                    let mut err =
                        ApiError::new(ErrorKind::Serialization, "re-encode envelope data");
                    err.set_source_internal(Some(Box::new(e)));
                    err
                });
            }
        }
    }
    Ok(raw.to_vec())
}

fn trim_leading_whitespace(b: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t' || b[i] == b'\n' || b[i] == b'\r') {
        i += 1;
    }
    &b[i..]
}

/// Compute the delay before retry attempt+1. Honors `Retry-After` (integer
/// seconds), but caps the honored value at 8 seconds so a hostile or buggy
/// server cannot park the client.
fn backoff_delay(attempt: u32, retry_after_secs: Option<u64>) -> Duration {
    if let Some(secs) = retry_after_secs {
        return Duration::from_secs(secs.min(8));
    }
    let base_ms: u64 = 500;
    let shifted = base_ms.saturating_mul(1u64 << attempt.min(4));
    Duration::from_millis(shifted.min(8_000))
}

/// Percent-encode a single URL path segment. Everything outside the RFC 3986
/// unreserved set (`A-Z a-z 0-9 - _ . ~`) is percent-encoded — in particular
/// `/`, which would otherwise let a caller-supplied value escape its segment.
pub(crate) fn encode_path_segment(s: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char)
            }
            _ => {
                out.push('%');
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_segment_leaves_unreserved_untouched() {
        assert_eq!(encode_path_segment("2015550123"), "2015550123");
        assert_eq!(encode_path_segment("w1476"), "w1476");
        assert_eq!(encode_path_segment("a-b_c.d~e"), "a-b_c.d~e");
    }

    #[test]
    fn path_segment_encodes_reserved_and_utf8() {
        assert_eq!(encode_path_segment("a/b"), "a%2Fb");
        assert_eq!(encode_path_segment("a b"), "a%20b");
        assert_eq!(encode_path_segment("a?b#c"), "a%3Fb%23c");
        assert_eq!(encode_path_segment("é"), "%C3%A9");
    }

    #[test]
    fn retry_after_is_capped_at_eight_seconds() {
        assert_eq!(backoff_delay(0, Some(0)), Duration::from_secs(0));
        assert_eq!(backoff_delay(0, Some(5)), Duration::from_secs(5));
        assert_eq!(backoff_delay(0, Some(8)), Duration::from_secs(8));
        assert_eq!(backoff_delay(0, Some(3600)), Duration::from_secs(8));
    }

    #[test]
    fn exponential_backoff_caps_at_eight_seconds() {
        assert_eq!(backoff_delay(0, None), Duration::from_millis(500));
        assert_eq!(backoff_delay(1, None), Duration::from_millis(1000));
        assert_eq!(backoff_delay(10, None), Duration::from_secs(8));
    }

    #[test]
    fn multipart_body_lays_out_fields_and_files() {
        let parts = vec![
            FormPart::field("fromNumber", "2015550123"),
            FormPart::file(
                "documents",
                "contract.pdf",
                "application/pdf",
                &b"%PDF-1.4 bytes"[..],
            ),
        ];
        let body = build_multipart_body(&parts, "voicetel-test-boundary");
        let text = String::from_utf8(body.clone()).unwrap();
        assert_eq!(
            text,
            "--voicetel-test-boundary\r\n\
             Content-Disposition: form-data; name=\"fromNumber\"\r\n\
             \r\n\
             2015550123\r\n\
             --voicetel-test-boundary\r\n\
             Content-Disposition: form-data; name=\"documents\"; filename=\"contract.pdf\"\r\n\
             Content-Type: application/pdf\r\n\
             \r\n\
             %PDF-1.4 bytes\r\n\
             --voicetel-test-boundary--\r\n"
        );
    }

    #[test]
    fn multipart_body_escapes_header_metacharacters() {
        let parts = vec![FormPart::file(
            "documents",
            "qu\"ote\\name.pdf",
            "application/pdf",
            b"x",
        )];
        let text = String::from_utf8(build_multipart_body(&parts, "b")).unwrap();
        assert!(text.contains("filename=\"qu\\\"ote\\\\name.pdf\""));
    }

    #[test]
    fn multipart_file_part_carries_binary_payload_intact() {
        let data = vec![0x00, 0xff, 0x0a, 0x0d, 0x25, 0x50];
        let parts = vec![FormPart::file(
            "documents",
            "a.pdf",
            "application/pdf",
            &data,
        )];
        let body = build_multipart_body(&parts, "b");
        let suffix = b"\r\n--b--\r\n";
        let end = body.len() - suffix.len();
        assert_eq!(&body[end - data.len()..end], &data[..]);
    }
}
