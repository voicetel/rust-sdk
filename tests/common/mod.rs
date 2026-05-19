//! Shared helpers for the wiremock-driven test suite.

#![allow(dead_code)]

use voicetel::Client;
use wiremock::MockServer;

/// Build a Client pointed at the mock server with an installed API key.
pub async fn test_client(server: &MockServer) -> Client {
    Client::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .max_retries(0)
        .build()
        .expect("build client")
}

/// Build a Client without an API key for testing the unauthenticated error path.
pub fn no_key_client(server: &MockServer) -> Client {
    Client::builder()
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .expect("build client")
}

/// Helper to wrap a JSON `data` payload in the `{status, data}` envelope.
pub fn envelope(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"status": "success", "data": data})
}
