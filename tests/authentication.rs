//! `Authentication` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::authentication::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn auth_get() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/auth"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "authType": 1, "authTypeDescription": "IP Auth", "acl": []
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.authentication().get().await.unwrap();
    assert_eq!(out.auth_type, 1);
}

#[tokio::test]
async fn auth_update() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/auth"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "updated": [{"field": "authType", "value": 1}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AuthPutRequest {
        auth_type: Some(auth_type::IP_AUTH),
        password: None,
    };
    let out = c.authentication().update(&body).await.unwrap();
    assert_eq!(out.updated[0].field, "authType");
    assert_eq!(out.updated[0].value, Some(1));
}
