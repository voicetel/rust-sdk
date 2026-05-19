//! `Lookups` resource tests.

mod common;
use common::{envelope, test_client};

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn cnam_lookup() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/cnam/2015550123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "cnam": "JOE PUBLIC"
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().cnam("2015550123").await.unwrap();
    assert_eq!(out.cnam.as_deref(), Some("JOE PUBLIC"));
}

#[tokio::test]
async fn lrn_lookup() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/lrn/2015550123/2015550100"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "ani": "2015550100", "destination": "2015550123",
                "lrn": {"lrn": "2015550100", "state": "NJ"}
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().lrn("2015550123", "2015550100").await.unwrap();
    assert_eq!(out.ani, "2015550100");
    assert_eq!(out.lrn.lrn.as_deref(), Some("2015550100"));
}
