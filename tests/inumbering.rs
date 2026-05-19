//! `iNumbering` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::inumbering::*;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn search_inventory_passes_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/inventory"))
        .and(query_param("npa", "201"))
        .and(query_param("state", "NJ"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "numbers": [{"number": "2015551234", "rateCenter": "ENGLEWOOD", "city": "Engl", "province": "NJ", "lata": "224"}]
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let q = InventoryQuery {
        npa: Some(201),
        state: Some("NJ".into()),
        ..Default::default()
    };
    let out = c.i_numbering().search_inventory(&q).await.unwrap();
    assert_eq!(out.numbers[0].number, "2015551234");
}

#[tokio::test]
async fn coverage_default() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/inventory/coverage"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "coverage": [{"count": 5, "npa": "201", "nxx": "555"}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .i_numbering()
        .coverage(&CoverageQuery::default())
        .await
        .unwrap();
    assert_eq!(out.coverage[0].count, 5);
}

#[tokio::test]
async fn order_string_or_object() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/orders"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "orderId": "ord_1",
                "amountCharged": 1.5,
                "numbersOrdered": ["2015551234", "2015555678"]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = OrderCreateRequest {
        numbers: vec![
            OrderNumber::plain("2015551234"),
            OrderNumber::with_route("2015555678", 5),
        ],
    };
    let out = c.i_numbering().order(&body).await.unwrap();
    assert_eq!(out.numbers_ordered.len(), 2);
}

#[tokio::test]
async fn ports_and_port() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/ports"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "ports": [{"status": "active", "id": "1"}]
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/ports/1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "port": {"status": "active"}
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let list = c.i_numbering().ports().await.unwrap();
    assert_eq!(list.ports.len(), 1);
    let one = c.i_numbering().port(1).await.unwrap();
    assert_eq!(one.port.status, "active");
}

#[tokio::test]
async fn submit_port() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/ports"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "pid": "AAAAA", "ticket": 7, "message": "ok", "loaUrl": "http://l", "portUrl": "http://p"
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = PortSubmitRequest {
        did: vec!["2015551234".into()],
        name: "Joe".into(),
        name_type: "residential".into(),
        lc_btn: "2015551200".into(),
        lc_account_number: "ACC".into(),
        street_number: "1".into(),
        street: "Main".into(),
        street_type: "ST".into(),
        city: "Engl".into(),
        state: "NJ".into(),
        zip: "07000".into(),
        country: "US".into(),
        auth_person: "Joe".into(),
        ..Default::default()
    };
    let out = c.i_numbering().submit_port(&body).await.unwrap();
    assert_eq!(out.pid, "AAAAA");
    assert_eq!(out.ticket, 7);
}

#[tokio::test]
async fn port_availability_includes_v2_2_10_fields() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/ports/availability/2015551234"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015551234",
                "portable": true,
                "losingCarrier": "AT&T",
                "localRoutingNumber": "2015550100",
                "rateCenterTier": "tier1",
                "reason": null
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .i_numbering()
        .port_availability("2015551234")
        .await
        .unwrap();
    assert!(out.portable);
    assert_eq!(out.local_routing_number.as_deref(), Some("2015550100"));
    assert_eq!(out.rate_center_tier.as_deref(), Some("tier1"));
}
