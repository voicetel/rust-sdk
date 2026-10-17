//! Transport-layer behaviour: auth header, retries, envelope unwrap, errors.

mod common;
use common::{envelope, test_client};

/// Build a Client without an API key for testing the unauthenticated error path.
fn no_key_client(server: &wiremock::MockServer) -> Client {
    Client::builder()
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .expect("build client")
}

use voicetel::{ApiError, Client, ErrorKind};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn login_installs_bearer_and_returns_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/account/api-key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "apikey": "deadbeef"
            }))),
        )
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .unwrap();
    let key = client.login("1000000001", "hunter2").await.unwrap();
    assert_eq!(key, "deadbeef");
    assert_eq!(client.api_key().as_deref(), Some("deadbeef"));
}

#[tokio::test]
async fn login_empty_key_yields_authentication_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/account/api-key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "apikey": ""
            }))),
        )
        .mount(&server)
        .await;
    let client = Client::builder()
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .unwrap();
    let err: ApiError = client.login("1", "x").await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Authentication);
}

#[tokio::test]
async fn authenticated_request_sends_bearer() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "username": "1000000001"
            }))),
        )
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let me = client.account().get().await.unwrap();
    assert_eq!(me.username.as_deref(), Some("1000000001"));
}

#[tokio::test]
async fn missing_api_key_errors_before_request() {
    let server = MockServer::start().await;
    let client = no_key_client(&server);
    let err = client.account().get().await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Authentication);
}

#[tokio::test]
async fn unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/acl"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "acl": [{"cidr": "203.0.113.0/24"}]
            }))),
        )
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let acl = client.acl().list().await.unwrap();
    assert_eq!(acl.acl.len(), 1);
    assert_eq!(acl.acl[0].cidr, "203.0.113.0/24");
}

#[tokio::test]
async fn maps_404_to_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/9999999999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "status": "error",
            "error": {"code": 404, "message": "no such TN"}
        })))
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let err = client.numbers().get("9999999999").await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::NotFound);
    assert!(err.is_not_found());
    assert_eq!(err.status_code, 404);
    assert_eq!(err.code, Some(404));
    assert_eq!(err.message, "no such TN");
}

#[tokio::test]
async fn parses_spec_error_envelope() {
    // Canonical spec shape: {"status":"error","error":{"code":<int>,"message":<string>}}
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "status": "error",
            "error": {"code": 401, "message": "Missing or invalid Bearer token"}
        })))
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let err = client.account().get().await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Authentication);
    assert_eq!(err.code, Some(401));
    assert_eq!(err.message, "Missing or invalid Bearer token");
    assert!(err
        .body
        .as_deref()
        .unwrap()
        .contains("\"status\":\"error\""));
}

#[tokio::test]
async fn auth_put_conflict_accessor_decodes_typed_data() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/auth"))
        .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
            "status": "error",
            "error": {"code": 409, "message": "One or more updates failed"},
            "updated": [{"field": "authType", "value": 1}],
            "errors": [{"field": "password", "message": "Must be 6-10 alphanumeric characters with at least one letter and one number"}]
        })))
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let body = voicetel::models::authentication::AuthPutRequest {
        auth_type: Some(1),
        password: Some("weak".into()),
    };
    let err = client.authentication().update(&body).await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Conflict);
    assert_eq!(err.code, Some(409));
    let detail = err.auth_put_conflict().expect("typed auth conflict data");
    let updated = detail.updated.as_deref().expect("updated fields");
    assert_eq!(updated[0].field, "authType");
    assert_eq!(updated[0].value, Some(1));
    let errors = detail.errors.as_deref().expect("error entries");
    assert_eq!(errors[0].field, "password");
    assert!(errors[0].message.contains("6-10 alphanumeric"));
    // A non-409 error yields no typed data.
    let other = ApiError::from_status(400, None, "x", None);
    assert!(other.auth_put_conflict().is_none());
    assert!(other.acl_conflict().is_none());
}

#[tokio::test]
async fn maps_429_to_rate_limit_with_no_retries() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .respond_with(ResponseTemplate::new(429).set_body_string("rate limited"))
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let err = client.account().get().await.unwrap_err();
    assert!(err.is_rate_limit());
    assert_eq!(err.status_code, 429);
}

#[tokio::test]
async fn retries_on_5xx_then_succeeds() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "username": "ok"
            }))),
        )
        .mount(&server)
        .await;
    let client = Client::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .max_retries(2)
        .build()
        .unwrap();
    let me = client.account().get().await.unwrap();
    assert_eq!(me.username.as_deref(), Some("ok"));
}

#[tokio::test]
async fn error_display_contains_status() {
    let err = ApiError::from_status(401, Some(401), "bad token", None);
    let s = format!("{}", err);
    assert!(s.contains("401"));
    assert!(s.contains("bad token"));
}

#[tokio::test]
async fn error_kind_helpers() {
    let e401 = ApiError::from_status(401, None, "x", None);
    assert!(e401.is_authentication());
    let e409 = ApiError::from_status(409, None, "x", None);
    assert!(e409.is_conflict());
    let e500 = ApiError::from_status(500, None, "x", None);
    assert_eq!(e500.kind, ErrorKind::Server);
    let e999 = ApiError::from_status(999, None, "x", None);
    assert_eq!(e999.kind, ErrorKind::Unknown);
}
