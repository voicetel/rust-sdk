//! `Gateways` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::gateways::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn gateways_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/gateways"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "gateways": [{"id": 1, "gateway": "1.2.3.4", "system": false, "limit": 23}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.gateways().list().await.unwrap();
    assert_eq!(out.gateways.len(), 1);
    assert_eq!(out.gateways[0].id, Some(1));
}

#[tokio::test]
async fn gateways_add_get_update_remove() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/gateways"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": 9, "gateway": "1.2.3.4", "limit": 10, "system": false
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/gateways/9"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": 9, "gateway": "1.2.3.4", "limit": 10, "system": false
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/gateways/9"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": 9, "gateway": "1.2.3.5", "limit": 25, "system": false
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/gateways/9"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let c = test_client(&server).await;

    let added = c
        .gateways()
        .add(&GatewayAddRequest {
            gateway: "1.2.3.4".into(),
            prefix: None,
            limit: Some(10),
        })
        .await
        .unwrap();
    assert_eq!(added.id, Some(9));

    let got = c.gateways().get(9).await.unwrap();
    assert_eq!(got.limit, Some(10));

    let upd = c
        .gateways()
        .update(
            9,
            &GatewayUpdateRequest {
                gateway: Some("1.2.3.5".into()),
                prefix: None,
                limit: Some(25),
            },
        )
        .await
        .unwrap();
    assert_eq!(upd.limit, Some(25));

    c.gateways().remove(9).await.unwrap();
}

#[tokio::test]
async fn gateways_numbers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/gateways/3/numbers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "numbers": [{"number": "2015551234", "translated": "2015551234", "forward": false, "forwardTo": null, "cnam": false, "carrier": 0, "smsEnabled": true, "faxEnabled": false}]
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.gateways().numbers(3).await.unwrap();
    assert_eq!(out.numbers[0].number, "2015551234");
}
