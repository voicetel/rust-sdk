//! `Support` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::support::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn support_list_get_create_update_delete() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/support/tickets"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "tickets": [{"status": "active", "number": 1015}]
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2.2/support/tickets/7"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "ticket": {"status": "active", "number": 1015}
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2.2/support/tickets"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "ticket": {"status": "active", "subject": "x", "number": 2222}
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2.2/support/tickets/7"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": 7, "status": "success"
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/support/tickets/7"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let c = test_client(&server).await;

    let list = c.support().list().await.unwrap();
    assert_eq!(list.tickets[0].ticket_number, Some(1015));

    let one = c.support().get(7).await.unwrap();
    assert_eq!(one.ticket.ticket_number, Some(1015));

    let created = c
        .support()
        .create(&TicketCreateRequest {
            subject: "x".into(),
            message: "hi".into(),
            email: None,
        })
        .await
        .unwrap();
    assert_eq!(created.ticket.ticket_number, Some(2222));

    let upd = c
        .support()
        .update(
            7,
            &TicketUpdateRequest {
                status: "closed".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(upd.status, "success");

    c.support().delete(7).await.unwrap();
}

#[tokio::test]
async fn support_messages_reply() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/support/tickets/7/messages"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "messages": [{"status": "active", "body": "hi"}]
            }))),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2.2/support/tickets/7/replies"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "message": "Reply added"
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let m = c.support().messages(7).await.unwrap();
    assert_eq!(m.messages.len(), 1);
    let r = c
        .support()
        .reply(
            7,
            &TicketReplyRequest {
                message: "ack".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(r.message, "Reply added");
}
