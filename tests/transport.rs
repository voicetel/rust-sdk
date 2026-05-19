//! Transport-layer behaviour: auth header, retries, envelope unwrap, errors.

mod common;
use common::{envelope, no_key_client, test_client};

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
    let key = client.login(1_000_000_001, "hunter2").await.unwrap();
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
    let err: ApiError = client.login(1, "x").await.unwrap_err();
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
            "code": "not_found",
            "message": "no such TN"
        })))
        .mount(&server)
        .await;
    let client = test_client(&server).await;
    let err = client.numbers().get("9999999999").await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::NotFound);
    assert!(err.is_not_found());
    assert_eq!(err.status_code, 404);
    assert_eq!(err.code.as_deref(), Some("not_found"));
    assert_eq!(err.message, "no such TN");
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
    let err = ApiError::from_status(401, Some("auth".into()), "bad token", None);
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
