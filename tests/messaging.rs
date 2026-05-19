//! `Messaging` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::messaging::*;
use wiremock::matchers::{body_partial_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn message_send_uses_camelcase_wire_fields() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/messages"))
        .and(body_partial_json(serde_json::json!({
            "fromNumber": "2015551234",
            "toNumber": "2015555678",
            "text": "hi"
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": "msg_1", "type": "sms",
                "fromNumber": "2015551234", "toNumber": "2015555678",
                "parts": 1
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = MessageSendRequest {
        from_number: "2015551234".into(),
        to_number: "2015555678".into(),
        text: "hi".into(),
        subject: None,
        media_urls: vec![],
    };
    let out = c.messaging().send(&body).await.unwrap();
    assert_eq!(out.id, "msg_1");
    assert_eq!(out.from_number, "2015551234");
}

#[tokio::test]
async fn message_history() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/messages"))
        .and(query_param("number", "2015551234"))
        .and(query_param("type", "sms"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "type": "sms",
                "fromTs": 0, "toTs": 1, "messages": []
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let opts = HistoryOptions {
        number: Some("2015551234".into()),
        kind: Some("sms".into()),
        ..Default::default()
    };
    let out = c.messaging().history(&opts).await.unwrap();
    assert_eq!(out.kind, "sms");
}

#[tokio::test]
async fn brand_and_campaign() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/messaging/brands"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "result": {"statusCode": "200", "status": "Success"}
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2.2/messaging/campaigns"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "result": {"statusCode": "200", "status": "Success"}
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/messaging/campaigns"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "campaigns": [{"id": "ABCDEFG", "status": "ACTIVE", "numbers": []}]
            }))),
        )
        .mount(&server)
        .await;

    let c = test_client(&server).await;

    let brand = c
        .messaging()
        .create_brand(&MessagingBrandCreateRequest {
            messaging_brand_id: "B123".into(),
            messaging_brand_name: "Acme".into(),
            messaging_brand_description: None,
        })
        .await
        .unwrap();
    assert_eq!(brand.result.status, "Success");

    let camp = c
        .messaging()
        .create_campaign(&MessagingCampaignCreateRequest {
            messaging_brand_id: "B123".into(),
            external_campaign_id: "X".into(),
            campaign_description: "Y".into(),
            campaign_class_name: None,
            campaign_start_date: None,
        })
        .await
        .unwrap();
    assert_eq!(camp.result.status, "Success");

    let status = c.messaging().campaign_status().await.unwrap();
    assert_eq!(status.campaigns.len(), 1);
}

#[tokio::test]
async fn numbers_state() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/messaging"))
        .and(query_param("numbers", "2015551234,2015555678"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "numbers": [
                {"number": "2015551234", "enabled": true, "carrier": 17, "routeIn": 1, "resource": "x", "network": "A", "campaign": null}
            ]
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .messaging()
        .numbers_state(&["2015551234".into(), "2015555678".into()])
        .await
        .unwrap();
    assert_eq!(out.numbers.len(), 1);
}
