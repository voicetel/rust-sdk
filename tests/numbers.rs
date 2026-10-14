//! `Numbers` resource tests — covers every method.

mod common;
use common::{envelope, test_client};

use voicetel::models::numbers::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn numbers_list_get() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "numbers": [{"number": "2015550123", "translated": "2015550123", "route": 4, "gateway": null, "cnam": false, "forward": false, "forwardTo": null, "carrier": 0, "smsEnabled": false, "faxEnabled": false}]
        }))))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/2015550123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015550123", "translated": "2015550123", "route": 4, "gateway": null, "cnam": false, "forward": false, "forwardTo": null, "carrier": 0, "smsEnabled": false, "faxEnabled": false
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let list = c.numbers().list().await.unwrap();
    assert_eq!(list.numbers.len(), 1);
    let one = c.numbers().get("2015550123").await.unwrap();
    assert_eq!(one.number, "2015550123");
}

#[tokio::test]
async fn numbers_add() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/numbers"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "route": 4
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .numbers()
        .add(&NumberAddRequest {
            number: "2015550123".into(),
            route: None,
        })
        .await
        .unwrap();
    assert_eq!(out.route, 4);
}

#[tokio::test]
async fn numbers_remove_204() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/numbers/2015550123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    c.numbers().remove("2015550123").await.unwrap();
}

#[tokio::test]
async fn numbers_move_release() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v2.2/numbers/2015550123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "accountId": 9, "route": 4
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2.2/numbers/2015550123/release"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let mv = c
        .numbers()
        .move_to(
            "2015550123",
            &NumberMoveRequest {
                account_id: 9,
                route: 4,
            },
        )
        .await
        .unwrap();
    assert_eq!(mv.account_id, 9);
    c.numbers().release("2015550123").await.unwrap();
}

#[tokio::test]
async fn numbers_set_route_translation_cnam_lidb() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015550123/route"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "route": 5
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015550123/translation"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "translation": "5551234"
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015550123/cnam"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "cnam": true
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015550123/lidb"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015550123", "cnam": "ACME", "customerOrderReference": "ref", "carrierStatus": "Success"
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let r = c
        .numbers()
        .set_route("2015550123", &NumberRouteRequest { route: 5 })
        .await
        .unwrap();
    assert_eq!(r.route, 5);
    let t = c
        .numbers()
        .set_translation(
            "2015550123",
            &NumberTranslationRequest {
                translation: "5551234".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(t.translation, "5551234");
    let cn = c
        .numbers()
        .set_cnam("2015550123", &NumberCnamRequest { enabled: true })
        .await
        .unwrap();
    assert!(cn.cnam);
    let l = c
        .numbers()
        .set_lidb(
            "2015550123",
            &NumberLidbRequest {
                cnam: "ACME".into(),
                customer_order_reference: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(l.cnam, "ACME");
}

#[tokio::test]
async fn numbers_fax_forward_sms_lifecycle() {
    let server = MockServer::start().await;
    for (m, p, body) in [
        (
            "GET",
            "/v2.2/numbers/2015550123/fax",
            serde_json::json!({"number": "2015550123", "email": "a@b"}),
        ),
        (
            "PUT",
            "/v2.2/numbers/2015550123/fax",
            serde_json::json!({"number": "2015550123", "email": "a@b"}),
        ),
        (
            "PUT",
            "/v2.2/numbers/2015550123/forward",
            serde_json::json!({"number": "2015550123", "forwardTo": "2015555678"}),
        ),
        (
            "GET",
            "/v2.2/numbers/2015550123/sms",
            serde_json::json!({"number": "2015550123", "type": "email", "resource": "a@b"}),
        ),
        (
            "PUT",
            "/v2.2/numbers/2015550123/sms",
            serde_json::json!({"number": "2015550123", "type": "webhook", "resource": "https://x"}),
        ),
    ] {
        Mock::given(method(m))
            .and(path(p))
            .respond_with(ResponseTemplate::new(200).set_body_json(envelope(body)))
            .mount(&server)
            .await;
    }
    for (m, p) in [
        ("DELETE", "/v2.2/numbers/2015550123/fax"),
        ("DELETE", "/v2.2/numbers/2015550123/forward"),
        ("DELETE", "/v2.2/numbers/2015550123/sms"),
    ] {
        Mock::given(method(m))
            .and(path(p))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
    }

    let c = test_client(&server).await;

    c.numbers().get_fax("2015550123").await.unwrap();
    c.numbers()
        .set_fax(
            "2015550123",
            &NumberFaxRequest {
                email: "a@b".into(),
            },
        )
        .await
        .unwrap();
    c.numbers().remove_fax("2015550123").await.unwrap();

    let f = c
        .numbers()
        .set_forward(
            "2015550123",
            &NumberForwardRequest {
                destination: "2015555678".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(f.forward_to.as_deref(), Some("2015555678"));
    c.numbers().remove_forward("2015550123").await.unwrap();

    c.numbers().get_sms("2015550123").await.unwrap();
    let s = c
        .numbers()
        .set_sms(
            "2015550123",
            &NumberSmsRequest {
                kind: "webhook".into(),
                resource: "https://x".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(s.kind, "webhook");
    c.numbers().remove_sms("2015550123").await.unwrap();
}

#[tokio::test]
async fn numbers_messaging_lifecycle() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/2015550123/messaging"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015550123", "enabled": true, "carrier": 1, "routeIn": 1, "resource": "x",
            "network": "C", "campaign": null,
            "pending": {
                "campaignId": "CAMP002", "state": "assigning", "message": "provisioning",
                "createdAt": "2026-10-01T12:00:00Z", "updatedAt": "2026-10-01T12:01:00Z"
            }
        }))))
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v2.2/numbers/2015550123/messaging"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "updated": ["routeIn"]
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015550123/messaging-campaign"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "campaignId": "CAMP002", "status": "pending",
                "carrier": 1, "network": "C", "upstreamCnpId": "BANDW",
                "previousNetwork": null, "previousNetworkCleared": false,
                "alreadyAssigned": false, "orderId": null, "warning": null,
                "message": "accepted for provisioning",
                "job": {
                    "campaignId": "CAMP002", "state": "queued", "message": "accepted",
                    "createdAt": "2026-10-01T12:00:00Z", "updatedAt": "2026-10-01T12:00:00Z"
                }
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/numbers/2015550123/messaging-campaign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015550123", "campaignId": "CAMP002", "network": "C", "upstreamCnpId": "BANDW", "unassigned": true
        }))))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/numbers/messaging-campaign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "campaignId": "CAMP002", "network": "C", "upstreamCnpId": "BANDW", "unassignedNumbers": ["2015550123"], "failed": []
        }))))
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v2.2/numbers/2015550123/port-out-pin"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "portOutPin": "1234", "changed": true
            }))),
        )
        .mount(&server)
        .await;

    let c = test_client(&server).await;
    let m = c.numbers().get_messaging("2015550123").await.unwrap();
    assert_eq!(m.network.as_deref(), Some("C"));
    let pending = m.pending.as_ref().expect("pending job");
    assert_eq!(pending.campaign_id, "CAMP002");
    assert_eq!(pending.state, MessagingEnablementJobState::Assigning);
    let p = c
        .numbers()
        .patch_messaging(
            "2015550123",
            &NumberMessagingPatchRequest {
                route_in: Some(1),
                route_out: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(p.updated, vec!["routeIn"]);
    let a = c
        .numbers()
        .assign_campaign(
            "2015550123",
            &NumberCampaignAssignRequest {
                campaign_id: "CAMP002".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(a.campaign_id, "CAMP002");
    assert_eq!(a.status, CampaignAssignStatus::Pending);
    assert!(!a.already_assigned);
    assert_eq!(a.order_id, None);
    assert_eq!(a.message.as_deref(), Some("accepted for provisioning"));
    assert_eq!(
        a.job.as_ref().unwrap().state,
        MessagingEnablementJobState::Queued
    );
    let u = c.numbers().unassign_campaign("2015550123").await.unwrap();
    assert!(u.unassigned);
    let bu = c
        .numbers()
        .bulk_unassign_campaign(vec!["2015550123".into()])
        .await
        .unwrap();
    assert_eq!(bu.unassigned_numbers.len(), 1);
    let pin = c
        .numbers()
        .set_port_out_pin(
            "2015550123",
            &PortOutPinUpdateRequest { pin: "1234".into() },
        )
        .await
        .unwrap();
    assert_eq!(pin.port_out_pin, "1234");
    assert!(pin.changed);
}

#[tokio::test]
async fn numbers_assign_campaign_already_and_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015550123/messaging-campaign"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123", "campaignId": "CAMP002", "status": "already",
                "carrier": 19, "network": "B", "upstreamCnpId": "CAMP003",
                "previousNetwork": "A", "previousNetworkCleared": true,
                "alreadyAssigned": true, "orderId": 987654,
                "warning": "stale registration on path A removal requested",
                "message": null, "job": null
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let a = c
        .numbers()
        .assign_campaign(
            "2015550123",
            &NumberCampaignAssignRequest {
                campaign_id: "CAMP002".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(a.status, CampaignAssignStatus::Already);
    assert!(a.already_assigned);
    assert!(a.previous_network_cleared);
    assert_eq!(a.previous_network.as_deref(), Some("A"));
    assert_eq!(a.order_id, Some(OrderId::Numeric(987654)));
    assert_eq!(
        a.warning.as_deref(),
        Some("stale registration on path A removal requested")
    );
    assert!(a.job.is_none());
}
