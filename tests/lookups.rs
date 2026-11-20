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
async fn carrier_lookup() {
    // v2.2.20: carrier/line-type dip; field names are the upstream
    // carrier's (snake_case on the wire).
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/carrier/2015550123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123",
                "carrier": {
                    "name": "EXAMPLE CARRIER",
                    "type": "voip",
                    "mobile_country_code": null,
                    "mobile_network_code": null,
                    "error_code": null
                }
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().carrier("2015550123").await.unwrap();
    assert_eq!(out.number, "2015550123");
    assert_eq!(out.carrier.name.as_deref(), Some("EXAMPLE CARRIER"));
    assert_eq!(out.carrier.kind.as_deref(), Some("voip"));
    assert_eq!(out.carrier.mobile_country_code, None);
    assert_eq!(out.carrier.mobile_network_code, None);
    assert_eq!(out.carrier.error_code, None);
}

#[tokio::test]
async fn carrier_lookup_mobile_fields() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/carrier/2015550100"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550100",
                "carrier": {
                    "name": null,
                    "type": "mobile",
                    "mobile_country_code": "310",
                    "mobile_network_code": "456",
                    "error_code": 3
                }
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().carrier("2015550100").await.unwrap();
    assert_eq!(out.carrier.kind.as_deref(), Some("mobile"));
    assert_eq!(out.carrier.mobile_country_code.as_deref(), Some("310"));
    assert_eq!(out.carrier.mobile_network_code.as_deref(), Some("456"));
    assert_eq!(out.carrier.error_code, Some(3));
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
    assert_eq!(
        out.lrn.as_ref().expect("lrn dip").lrn.as_deref(),
        Some("2015550100")
    );
}

#[tokio::test]
async fn lrn_lookup_no_record() {
    // v2.2.21: a failed dip is 200 with lrn:null — still billed.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/lrn/2015550123/2015550100"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "ani": "2015550100", "destination": "2015550123", "lrn": null
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.lookups().lrn("2015550123", "2015550100").await.unwrap();
    assert_eq!(out.ani, "2015550100");
    assert!(out.lrn.is_none());
}
