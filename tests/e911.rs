//! `e911` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::e911::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn e911_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/e911"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "records": [{"dn": "12015551234", "callername": "Joe", "address1": "1 St", "city": "C", "state": "NJ", "zip": "07000"}]
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.e911().list().await.unwrap();
    assert_eq!(out.records.len(), 1);
}

#[tokio::test]
async fn e911_create() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/e911"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "record": {"dn": "12015551234", "callername": "Joe", "address1": "1 St", "city": "C", "state": "NJ", "zip": "07000"}
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = E911CreateRequest {
        dn: "2015551234".into(),
        callername: "Joe".into(),
        address1: "1 St".into(),
        address2: None,
        city: "C".into(),
        state: "NJ".into(),
        zip: "07000".into(),
    };
    let out = c.e911().create(&body).await.unwrap();
    assert_eq!(out.record.dn, "12015551234");
}

#[tokio::test]
async fn e911_validate() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/e911/validations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "address": {"addressid": 42, "address1": "1 St", "city": "C", "state": "NJ", "zip": "07000"}
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = E911AddressRequest {
        address1: "1 St".into(),
        address2: None,
        city: "C".into(),
        state: "NJ".into(),
        zip: "07000".into(),
    };
    let out = c.e911().validate(&body).await.unwrap();
    assert_eq!(out.address.addressid, 42);
}

#[tokio::test]
async fn e911_get() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/e911/2015551234"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "record": {"dn": "12015551234", "callername": "Joe", "address1": "1 St", "city": "C", "state": "NJ", "zip": "07000"}
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.e911().get("2015551234").await.unwrap();
    assert_eq!(out.record.callername, "Joe");
}

#[tokio::test]
async fn e911_provision() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/e911/2015551234"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "record": {"dn": "12015551234", "callername": "Joe", "address1": "1 St", "city": "C", "state": "NJ", "zip": "07000"}
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = E911ProvisionByIdRequest {
        callername: "Joe".into(),
        addressid: 42,
    };
    let out = c.e911().provision("2015551234", &body).await.unwrap();
    assert_eq!(out.record.dn, "12015551234");
}

#[tokio::test]
async fn e911_remove() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/e911/2015551234"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    c.e911().remove("2015551234").await.unwrap();
}
