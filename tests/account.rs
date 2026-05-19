//! `Account` resource — happy + error paths.

mod common;
use common::{envelope, test_client};

use voicetel::models::account::*;
use wiremock::matchers::{body_partial_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn account_get() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "username": "1000000001",
                "cash": 12.5,
                "rates": {"sms": 0.005, "mms": 0.02},
                "services": {"sms": true, "mms": false}
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let me = c.account().get().await.unwrap();
    assert_eq!(me.username.as_deref(), Some("1000000001"));
    assert_eq!(me.cash, Some(12.5));
    assert_eq!(me.rates.as_ref().unwrap().sms, Some(0.005));
    assert_eq!(me.services.as_ref().unwrap().sms, Some(true));
}

#[tokio::test]
async fn account_update() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/account"))
        .and(body_partial_json(
            serde_json::json!({"notify": true, "notifyThreshold": 10}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "updated": ["notify", "notifyThreshold"]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AccountPutRequest {
        notify: Some(true),
        notify_threshold: Some(10),
        ..Default::default()
    };
    let out = c.account().update(&body).await.unwrap();
    assert_eq!(out.updated, vec!["notify", "notifyThreshold"]);
}

#[tokio::test]
async fn account_add() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/account"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "username": "1000000099",
                "password": "tempPass1"
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AccountAddRequest {
        username: 1_000_000_099,
        name: "Sub".into(),
        email: "sub@example.com".into(),
        master_account: None,
    };
    let out = c.account().add(&body).await.unwrap();
    assert_eq!(out.username.as_deref(), Some("1000000099"));
    assert_eq!(out.password.as_deref(), Some("tempPass1"));
}

#[tokio::test]
async fn account_signup() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/accounts"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "username": "1000000200"
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AccountSignupRequest {
        name: "Joe".into(),
        email: "joe@example.com".into(),
        promo: Some("WELCOME".into()),
    };
    let out = c.account().signup(&body).await.unwrap();
    assert_eq!(out.username.as_deref(), Some("1000000200"));
}

#[tokio::test]
async fn account_cdr_includes_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account/cdr"))
        .and(query_param("start", "1"))
        .and(query_param("end", "2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "cdr": [],
                "start": 1,
                "end": 2
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let cdr = c.account().cdr(1, 2).await.unwrap();
    assert_eq!(cdr.start, 1);
    assert_eq!(cdr.end, 2);
}

#[tokio::test]
async fn account_credits() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account/credits"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "credits": [{"date": "2026-05-01", "paid": true, "amount": 10.0}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.account().credits().await.unwrap();
    assert_eq!(out.credits.len(), 1);
    assert_eq!(out.credits[0].amount, 10.0);
}

#[tokio::test]
async fn account_recurring_charges() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account/recurring-charges"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "charges": [{"amount": 1.5, "description": "DID"}],
                "total": 1.5
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.account().recurring_charges().await.unwrap();
    assert_eq!(out.total, 1.5);
}

#[tokio::test]
async fn account_payments() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account/payments"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "payments": [{"date": "2026-04-30", "status": "Completed", "amount": 25.0}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.account().payments().await.unwrap();
    assert_eq!(out.payments[0].status, "Completed");
}

#[tokio::test]
async fn account_registration() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account/registration"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "agent": "Acme/1.0",
                "uri": "sip:1@example",
                "expires": 3600
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.account().registration().await.unwrap();
    assert_eq!(out.expires, Some(3600));
}

#[tokio::test]
async fn account_recover_no_auth() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/account/recovery"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "message": "email sent"
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AccountRecoverRequest {
        email: "x@example.com".into(),
    };
    let out = c.account().recover(&body).await.unwrap();
    assert_eq!(out.message.as_deref(), Some("email sent"));
}

#[tokio::test]
async fn account_get_400_error_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/account"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "code": "bad", "message": "explode"
        })))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let err = c.account().get().await.unwrap_err();
    assert_eq!(err.status_code, 400);
}
