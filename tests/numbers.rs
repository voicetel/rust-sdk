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
            "numbers": [{"number": "2015551234", "translated": "2015551234", "route": 4, "gateway": null, "cnam": false, "forward": false, "forwardTo": null, "carrier": 0, "smsEnabled": false, "faxEnabled": false}]
        }))))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/2015551234"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015551234", "translated": "2015551234", "route": 4, "gateway": null, "cnam": false, "forward": false, "forwardTo": null, "carrier": 0, "smsEnabled": false, "faxEnabled": false
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let list = c.numbers().list().await.unwrap();
    assert_eq!(list.numbers.len(), 1);
    let one = c.numbers().get("2015551234").await.unwrap();
    assert_eq!(one.number, "2015551234");
}

#[tokio::test]
async fn numbers_add() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/numbers"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "route": 4
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .numbers()
        .add(&NumberAddRequest {
            number: "2015551234".into(),
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
        .and(path("/v2.2/numbers/2015551234"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    c.numbers().remove("2015551234").await.unwrap();
}

#[tokio::test]
async fn numbers_move_release() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v2.2/numbers/2015551234"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "accountId": 9, "route": 4
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2.2/numbers/2015551234/release"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let mv = c
        .numbers()
        .move_to(
            "2015551234",
            &NumberMoveRequest {
                account_id: 9,
                route: 4,
            },
        )
        .await
        .unwrap();
    assert_eq!(mv.account_id, 9);
    c.numbers().release("2015551234").await.unwrap();
}

#[tokio::test]
async fn numbers_set_route_translation_cnam_lidb() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015551234/route"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "route": 5
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015551234/translation"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "translation": "5551234"
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015551234/cnam"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "cnam": true
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015551234/lidb"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015551234", "cnam": "ACME", "customerOrderReference": "ref", "carrierStatus": "Success"
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let r = c
        .numbers()
        .set_route("2015551234", &NumberRouteRequest { route: 5 })
        .await
        .unwrap();
    assert_eq!(r.route, 5);
    let t = c
        .numbers()
        .set_translation(
            "2015551234",
            &NumberTranslationRequest {
                translation: "5551234".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(t.translation, "5551234");
    let cn = c
        .numbers()
        .set_cnam("2015551234", &NumberCnamRequest { enabled: true })
        .await
        .unwrap();
    assert!(cn.cnam);
    let l = c
        .numbers()
        .set_lidb(
            "2015551234",
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
            "/v2.2/numbers/2015551234/fax",
            serde_json::json!({"number": "2015551234", "email": "a@b"}),
        ),
        (
            "PUT",
            "/v2.2/numbers/2015551234/fax",
            serde_json::json!({"number": "2015551234", "email": "a@b"}),
        ),
        (
            "PUT",
            "/v2.2/numbers/2015551234/forward",
            serde_json::json!({"number": "2015551234", "forwardTo": "2015555678"}),
        ),
        (
            "GET",
            "/v2.2/numbers/2015551234/sms",
            serde_json::json!({"number": "2015551234", "type": "email", "resource": "a@b"}),
        ),
        (
            "PUT",
            "/v2.2/numbers/2015551234/sms",
            serde_json::json!({"number": "2015551234", "type": "webhook", "resource": "https://x"}),
        ),
    ] {
        Mock::given(method(m))
            .and(path(p))
            .respond_with(ResponseTemplate::new(200).set_body_json(envelope(body)))
            .mount(&server)
            .await;
    }
    for (m, p) in [
        ("DELETE", "/v2.2/numbers/2015551234/fax"),
        ("DELETE", "/v2.2/numbers/2015551234/forward"),
        ("DELETE", "/v2.2/numbers/2015551234/sms"),
    ] {
        Mock::given(method(m))
            .and(path(p))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
    }

    let c = test_client(&server).await;

    c.numbers().get_fax("2015551234").await.unwrap();
    c.numbers()
        .set_fax(
            "2015551234",
            &NumberFaxRequest {
                email: "a@b".into(),
            },
        )
        .await
        .unwrap();
    c.numbers().remove_fax("2015551234").await.unwrap();

    let f = c
        .numbers()
        .set_forward(
            "2015551234",
            &NumberForwardRequest {
                destination: "2015555678".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(f.forward_to.as_deref(), Some("2015555678"));
    c.numbers().remove_forward("2015551234").await.unwrap();

    c.numbers().get_sms("2015551234").await.unwrap();
    let s = c
        .numbers()
        .set_sms(
            "2015551234",
            &NumberSmsRequest {
                kind: "webhook".into(),
                resource: "https://x".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(s.kind, "webhook");
    c.numbers().remove_sms("2015551234").await.unwrap();
}

#[tokio::test]
async fn numbers_messaging_lifecycle() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/2015551234/messaging"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015551234", "enabled": true, "carrier": 17, "routeIn": 1, "resource": "x", "network": "A", "campaign": null
        }))))
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v2.2/numbers/2015551234/messaging"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "updated": ["routeIn"]
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/numbers/2015551234/messaging-campaign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015551234", "campaignId": "ABCDEFG", "carrier": 17, "network": "A", "upstreamCnpId": "SFL9UTQ", "previousNetwork": null, "previousNetworkCleared": false
        }))))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/numbers/2015551234/messaging-campaign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "number": "2015551234", "campaignId": "ABCDEFG", "network": "A", "upstreamCnpId": "SFL9UTQ", "unassigned": true
        }))))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/numbers/messaging-campaign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "campaignId": "ABCDEFG", "network": "A", "upstreamCnpId": "SFL9UTQ", "unassignedNumbers": ["2015551234"], "failed": []
        }))))
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v2.2/numbers/2015551234/port-out-pin"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234", "portOutPin": "1234"
            }))),
        )
        .mount(&server)
        .await;

    let c = test_client(&server).await;
    c.numbers().get_messaging("2015551234").await.unwrap();
    let p = c
        .numbers()
        .patch_messaging(
            "2015551234",
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
            "2015551234",
            &NumberCampaignAssignRequest {
                campaign_id: "ABCDEFG".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(a.campaign_id, "ABCDEFG");
    let u = c.numbers().unassign_campaign("2015551234").await.unwrap();
    assert!(u.unassigned);
    let bu = c
        .numbers()
        .bulk_unassign_campaign(vec!["2015551234".into()])
        .await
        .unwrap();
    assert_eq!(bu.unassigned_numbers.len(), 1);
    let pin = c
        .numbers()
        .set_port_out_pin(
            "2015551234",
            &PortOutPinUpdateRequest { pin: "1234".into() },
        )
        .await
        .unwrap();
    assert_eq!(pin.port_out_pin, "1234");
}
