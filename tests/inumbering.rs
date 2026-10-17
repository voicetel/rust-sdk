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
        .and(query_param("tnMask", "201555*"))
        .and(query_param("province", "NJ"))
        .and(query_param("quantity", "25"))
        .and(query_param("rateCenter", "CLOSTER"))
        .and(query_param("lata", "224"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "numbers": [{"number": "2015550123", "rateCenter": "ENGLEWOOD", "city": "Engl", "province": "NJ", "lata": "224"}]
        }))))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let q = InventoryQuery {
        tn_mask: "201555*".into(),
        quantity: Some(25),
        province: Some("NJ".into()),
        rate_center: Some("CLOSTER".into()),
        lata: Some("224".into()),
    };
    let out = c.i_numbering().search_inventory(&q).await.unwrap();
    assert_eq!(out.numbers[0].number, "2015550123");
}

#[tokio::test]
async fn coverage_sends_count_by_and_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/inventory/coverage"))
        .and(query_param("countBy", "npanxx"))
        .and(query_param("locState", "NJ"))
        .and(query_param("npa", "201"))
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
        .coverage(&CoverageQuery {
            count_by: CoverageCountBy::NpaNxx,
            loc_state: Some("NJ".into()),
            npa: Some("201".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(out.coverage[0].count, 5);
    assert_eq!(out.coverage[0].npa.as_deref(), Some("201"));
}

#[tokio::test]
async fn coverage_count_by_wire_values() {
    assert_eq!(CoverageCountBy::State.as_str(), "state");
    assert_eq!(CoverageCountBy::City.as_str(), "city");
    assert_eq!(CoverageCountBy::RateCenter.as_str(), "rateCenter");
    assert_eq!(CoverageCountBy::NpaNxx.as_str(), "npanxx");
    assert_eq!(CoverageCountBy::Block.as_str(), "block");
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
                "numbersOrdered": ["2015550123", "2015555678"]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = OrderCreateRequest {
        numbers: vec![
            OrderNumber::plain("2015550123"),
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
        .and(path("/v2.2/ports/w1476"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "port": {"status": "active", "pid": "w1476"}
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let list = c.i_numbering().ports().await.unwrap();
    assert_eq!(list.ports.len(), 1);
    let one = c.i_numbering().port("w1476").await.unwrap();
    assert_eq!(one.port.status, "active");
    assert_eq!(one.port.pid.as_deref(), Some("w1476"));
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
        did: vec!["2015550123".into()],
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
        .and(path("/v2.2/ports/availability/2015550123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123",
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
        .port_availability("2015550123")
        .await
        .unwrap();
    assert!(out.portable);
    assert_eq!(out.local_routing_number.as_deref(), Some("2015550100"));
    assert_eq!(out.rate_center_tier.as_deref(), Some("tier1"));
}
