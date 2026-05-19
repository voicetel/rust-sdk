//! `Lookups` resource tests.

mod common;
use common::{envelope, test_client};

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn cnam_lookup() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/cnam/2015551234"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "cnam": "JOE PUBLIC"
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().cnam("2015551234").await.unwrap();
    assert_eq!(out.cnam.as_deref(), Some("JOE PUBLIC"));
}

#[tokio::test]
async fn lrn_lookup() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/lrn/2015551234/2012548000"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "ani": "2012548000", "destination": "2015551234",
                "lrn": {"lrn": "2015550100", "state": "NJ"}
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().lrn("2015551234", "2012548000").await.unwrap();
    assert_eq!(out.ani, "2012548000");
    assert_eq!(out.lrn.lrn.as_deref(), Some("2015550100"));
}
