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
            "fromNumber": "2015550123",
            "toNumber": "2015555678",
            "text": "hi"
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": "msg_1", "type": "sms",
                "fromNumber": "2015550123", "toNumber": "2015555678",
                "parts": 1
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = MessageSendRequest {
        from_number: "2015550123".into(),
        to_number: "2015555678".into(),
        text: Some("hi".into()),
        subject: None,
        media_urls: vec![],
    };
    let out = c.messaging().send(&body).await.unwrap();
    assert_eq!(out.id, "msg_1");
    assert_eq!(out.from_number, "2015550123");
}

#[tokio::test]
async fn message_send_media_only_mms_omits_text() {
    // v2.2.20: text is optional — a media-only MMS sends no `text` field.
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/messages"))
        .and(body_partial_json(serde_json::json!({
            "fromNumber": "2015550123",
            "toNumber": "2015555678",
            "mediaUrls": ["https://example.com/photo.jpg"]
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": "msg_2", "type": "mms",
                "fromNumber": "2015550123", "toNumber": "2015555678",
                "parts": 1,
                "mediaUrls": ["https://example.com/photo.jpg"]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = MessageSendRequest {
        from_number: "2015550123".into(),
        to_number: "2015555678".into(),
        text: None,
        subject: None,
        media_urls: vec!["https://example.com/photo.jpg".into()],
    };
    let out = c.messaging().send(&body).await.unwrap();
    assert_eq!(out.kind, "mms");
    assert_eq!(
        out.media_urls,
        vec!["https://example.com/photo.jpg".to_string()]
    );
}

#[tokio::test]
async fn message_history() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/messages"))
        .and(query_param("number", "2015550123"))
        .and(query_param("type", "sms"))
        .and(query_param("from", "1747345200"))
        .and(query_param("to", "1747258800"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "type": "sms",
                "fromTs": 1747345200, "toTs": 1747258800,
                "messages": [{
                    "id": "41946b69",
                    "key": ["2015550123", 1747345200],
                    "value": {"direction": "in", "number": "2015550199", "message": "hi", "rate": "0.0150"}
                }]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let opts = HistoryOptions {
        number: "2015550123".into(),
        from: Some(1747345200),
        to: Some(1747258800),
        kind: Some("sms".into()),
    };
    let out = c.messaging().history(&opts).await.unwrap();
    assert_eq!(out.kind, "sms");
    // v2.2.15: the far-end `number` inside a message record is a string.
    assert_eq!(out.messages[0].value.number.as_deref(), Some("2015550199"));
}

#[tokio::test]
async fn message_history_record_v2_2_20_fields() {
    // v2.2.20: records carry messageText / transactionGUID /
    // concatenatedMessage (dlr), attachments / subject (mms), and ip
    // (outbound sms) — all nullable optionals.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/messages"))
        .and(query_param("number", "2015550123"))
        .and(query_param("type", "mms"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "type": "mms",
                "fromTs": 1747345200, "toTs": 1747258800,
                "messages": [
                    {
                        "id": "dlr1",
                        "key": ["1000000001", "12015550123", 1747345100],
                        "value": {
                            "sourceNumber": "12015550123",
                            "destinationNumber": "12015550100",
                            "messageText": "id:1 sub:001 stat:DELIVRD err:000 text:delivered",
                            "transactionGUID": "abc-123",
                            "concatenatedMessage": "2"
                        }
                    },
                    {
                        "id": "mms1",
                        "key": ["1000000001", "2015550123", 1747340000],
                        "value": {
                            "direction": "out",
                            "number": "2015550199",
                            "message": "",
                            "rate": "0.0250",
                            "subject": "Invoice",
                            "ip": "203.0.113.10",
                            "attachments": [
                                {"contentFile": "https://files.example.com/f.jpg",
                                 "Content-Type": "image/jpeg",
                                 "fileName": "f.jpg",
                                 "extension": "jpg"}
                            ]
                        }
                    }
                ]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let opts = HistoryOptions {
        number: "2015550123".into(),
        from: None,
        to: None,
        kind: Some("mms".into()),
    };
    let out = c.messaging().history(&opts).await.unwrap();
    let dlr = &out.messages[0].value;
    assert_eq!(dlr.source_number.as_deref(), Some("12015550123"));
    assert_eq!(
        dlr.message_text.as_deref(),
        Some("id:1 sub:001 stat:DELIVRD err:000 text:delivered")
    );
    assert_eq!(dlr.transaction_guid.as_deref(), Some("abc-123"));
    assert_eq!(dlr.concatenated_message.as_deref(), Some("2"));
    let mms = &out.messages[1].value;
    assert_eq!(mms.subject.as_deref(), Some("Invoice"));
    assert_eq!(mms.ip.as_deref(), Some("203.0.113.10"));
    let att = mms.attachments.as_ref().expect("attachments");
    assert_eq!(att[0]["fileName"], "f.jpg");
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
        .and(query_param("ids", "CAMP002,NOEXIST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "campaigns": [
                    {
                        "id": "CAMP002",
                        "status": "CAMPAIGN_DCA_COMPLETE",
                        "numbers": ["2015550123"],
                        "dca": {
                            "electionComplete": true,
                            "electedCarriers": ["AT&T", "T-Mobile", "Verizon Wireless"],
                            "carriers": [
                                {"carrier": "T-Mobile", "status": "ACTIVE", "detail": "Campaign approved", "updatedAt": "2026-10-01T12:00:00Z"}
                            ]
                        }
                    },
                    {"id": "NOEXIST", "status": "CAMPAIGN_INVALID_ID", "numbers": [], "dca": null}
                ]
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

    let status = c
        .messaging()
        .campaign_status(&["CAMP002".into(), "NOEXIST".into()])
        .await
        .unwrap();
    assert_eq!(status.campaigns.len(), 2);
    let ready = &status.campaigns[0];
    assert_eq!(ready.status, "CAMPAIGN_DCA_COMPLETE");
    let dca = ready.dca.as_ref().expect("dca present");
    assert!(dca.election_complete);
    assert_eq!(
        dca.elected_carriers,
        Some(
            vec!["AT&T", "T-Mobile", "Verizon Wireless"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        )
    );
    assert_eq!(dca.carriers.len(), 1);
    assert_eq!(dca.carriers[0].carrier, "T-Mobile");
    assert_eq!(dca.carriers[0].status, "ACTIVE");
    assert_eq!(dca.carriers[0].detail, "Campaign approved");
    assert_eq!(dca.carriers[0].updated_at, "2026-10-01T12:00:00Z");
    assert!(status.campaigns[1].dca.is_none());
}

#[tokio::test]
async fn numbers_state() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/messaging"))
        .and(query_param("numbers", "2015550123,2015555678"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "numbers": [
                {"number": "2015550123", "enabled": true, "carrier": 17, "routeIn": 1, "resource": "x", "network": "A", "campaign": null}
            ]
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .messaging()
        .numbers_state(&["2015550123".into(), "2015555678".into()])
        .await
        .unwrap();
    assert_eq!(out.numbers.len(), 1);
}

#[tokio::test]
async fn messaging_health_surface() {
    // v2.2.15: messaging-health endpoints have no response schema in the spec, so the
    // SDK returns the decoded JSON as a raw serde_json::Value; bodies/query are matched.
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2.2/messaging/health"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "summary": { "numbers": 3, "healthy": 2 }
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/messaging/health/alerts"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "enabled": true
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/messaging/health/alerts"))
        .and(body_partial_json(serde_json::json!({ "enabled": false })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "enabled": false
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2.2/messaging/health/check"))
        .and(body_partial_json(
            serde_json::json!({ "number": "2125550123" }),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2125550123", "inService": true
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/messaging/health/samples"))
        .and(query_param("number", "2125550123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "samples": [{ "reason": "spam" }]
            }))),
        )
        .mount(&server)
        .await;

    let c = test_client(&server).await;

    let health = c.messaging().health().await.unwrap();
    assert_eq!(health["summary"]["numbers"], 3);

    let alerts = c.messaging().health_alerts().await.unwrap();
    assert_eq!(alerts["enabled"], true);

    let off = c.messaging().set_health_alerts(false).await.unwrap();
    assert_eq!(off["enabled"], false);

    let check = c.messaging().health_check("2125550123").await.unwrap();
    assert_eq!(check["inService"], true);

    let samples = c.messaging().health_samples("2125550123").await.unwrap();
    assert_eq!(samples["samples"][0]["reason"], "spam");
}
