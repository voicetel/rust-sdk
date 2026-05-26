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
    /// `require_auth=false` skips the `Authorization` header — used only by
    /// the api-key exchange and the public password-recovery endpoint.
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
        // 204 No Content / empty body for methods returning `()` — handled
        // by the dedicated `request_no_content` path. Anything else with an
        // empty body is an error here.
        if raw.is_empty() {
            return Err(ApiError::new(
                ErrorKind::Serialization,
                "empty response body when one was expected",
            ));
        }
        let inner = unwrap_envelope(&raw)?;
        serde_json::from_slice::<T>(&inner).map_err(|e| {
            let mut err = ApiError::new(ErrorKind::Serialization, "decode response body");
            err.body = Some(String::from_utf8_lossy(&inner).into_owned());
            err.set_source_internal(Some(Box::new(e)));
            err
        })
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

    /// Inner request loop. Returns the raw response body bytes on 2xx,
    /// or an `ApiError` for any other outcome.
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
        if require_auth && self.api_key().is_none() {
            return Err(ApiError::new(
                ErrorKind::Authentication,
                "no api key set; call Client::login or use Client::builder().api_key(...)",
            ));
        }

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

        let mut attempt: u32 = 0;
        loop {
            let mut req = self
                .http
                .request(method.clone(), &url)
                .header(header::USER_AGENT, &self.user_agent)
                .header(header::ACCEPT, "application/json")
                .header(header::ACCEPT_ENCODING, "gzip");
            if let Some(ref bytes) = body_bytes {
                req = req
                    .header(header::CONTENT_TYPE, "application/json")
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
}

/// Inspect a final response: return bytes on 2xx; build an `ApiError` otherwise.
fn decode(status: StatusCode, body: Vec<u8>) -> Result<Vec<u8>, ApiError> {
    if status.is_success() {
        return Ok(body);
    }
    let text = String::from_utf8_lossy(&body).into_owned();
    let mut code = None;
    let mut message = format!("HTTP {}", status.as_u16());
    if let Ok(val) = serde_json::from_slice::<Value>(&body) {
        if let Some(obj) = val.as_object() {
            if let Some(c) = obj.get("code").and_then(|v| v.as_str()) {
                code = Some(c.to_string());
            } else if let Some(c) = obj.get("error").and_then(|v| v.as_str()) {
                code = Some(c.to_string());
            }
            if let Some(m) = obj.get("message").and_then(|v| v.as_str()) {
                message = m.to_string();
            } else if let Some(m) = obj.get("error").and_then(|v| v.as_str()) {
                message = m.to_string();
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

/// Compute the delay before retry attempt+1. Honors `Retry-After` (integer seconds).
fn backoff_delay(attempt: u32, retry_after_secs: Option<u64>) -> Duration {
    if let Some(secs) = retry_after_secs {
        return Duration::from_secs(secs);
    }
    let base_ms: u64 = 500;
    let shifted = base_ms.saturating_mul(1u64 << attempt.min(4));
    Duration::from_millis(shifted.min(8_000))
}
