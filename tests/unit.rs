//! Pure unit tests — error construction, model serialisation quirks, builder defaults.

use std::error::Error;

use voicetel::models::inumbering::{OrderNumber, PortAvailabilityData};
use voicetel::models::messaging::MessageSendRequest;
use voicetel::models::numbers::NumberMoveData;
use voicetel::models::support::SupportConversation;
use voicetel::{ApiError, Client, ErrorKind, DEFAULT_BASE_URL, SDK_VERSION};

#[test]
fn ordernumber_plain_serialises_as_string() {
    let n = OrderNumber::plain("2015551234");
    let v = serde_json::to_value(&n).unwrap();
    assert_eq!(v, serde_json::json!("2015551234"));
}

#[test]
fn ordernumber_with_route_serialises_as_object() {
    let n = OrderNumber::with_route("2015551234", 5);
    let v = serde_json::to_value(&n).unwrap();
    assert_eq!(v, serde_json::json!({"number": "2015551234", "route": 5}));
}

#[test]
fn ordernumber_deserialises_string() {
    let n: OrderNumber = serde_json::from_str("\"2015551234\"").unwrap();
    assert_eq!(n, OrderNumber::Plain("2015551234".into()));
}

#[test]
fn ordernumber_deserialises_object() {
    let n: OrderNumber =
        serde_json::from_str("{\"number\": \"2015551234\", \"route\": 4}").unwrap();
    match n {
        OrderNumber::WithRoute { number, route } => {
            assert_eq!(number, "2015551234");
            assert_eq!(route, Some(4));
        }
        _ => panic!("expected WithRoute"),
    }
}

#[test]
fn ordernumber_deserialises_object_without_route() {
    let n: OrderNumber = serde_json::from_str("{\"number\": \"2015551234\"}").unwrap();
    if let OrderNumber::WithRoute { route, .. } = n {
        assert!(route.is_none());
    } else {
        panic!("expected WithRoute");
    }
}

#[test]
fn ordernumber_rejects_non_string_or_object() {
    let res: Result<OrderNumber, _> = serde_json::from_str("42");
    assert!(res.is_err());
}

#[test]
fn message_send_request_renames_fields() {
    let body = MessageSendRequest {
        from_number: "2015551234".into(),
        to_number: "2015555678".into(),
        text: "hi".into(),
        subject: None,
        media_urls: vec![],
    };
    let v = serde_json::to_value(&body).unwrap();
    assert!(v.get("fromNumber").is_some());
    assert!(v.get("toNumber").is_some());
    assert!(v.get("from_number").is_none());
}

#[test]
fn support_conversation_ticket_number_renames() {
    let raw = r#"{"status": "active", "number": 1015}"#;
    let conv: SupportConversation = serde_json::from_str(raw).unwrap();
    assert_eq!(conv.ticket_number, Some(1015));
    // Make sure we serialise back to `number`.
    let v = serde_json::to_value(&conv).unwrap();
    assert_eq!(v.get("number").and_then(|x| x.as_i64()), Some(1015));
    assert!(v.get("ticketNumber").is_none());
}

#[test]
fn number_move_data_renames_account_id() {
    let m = NumberMoveData {
        number: "2015551234".into(),
        account_id: 9,
        route: 4,
    };
    let v = serde_json::to_value(&m).unwrap();
    assert!(v.get("accountId").is_some());
    assert!(v.get("account_id").is_none());
}

#[test]
fn port_availability_v2_2_10_field_renames() {
    let raw = r#"{
        "number": "2015551234",
        "portable": true,
        "losingCarrier": "AT&T",
        "localRoutingNumber": "2015550100",
        "rateCenterTier": "tier1",
        "reason": null
    }"#;
    let data: PortAvailabilityData = serde_json::from_str(raw).unwrap();
    assert_eq!(data.local_routing_number.as_deref(), Some("2015550100"));
    assert_eq!(data.rate_center_tier.as_deref(), Some("tier1"));
    assert_eq!(data.losing_carrier.as_deref(), Some("AT&T"));
}

#[test]
fn errorkind_from_status_maps_known_codes() {
    assert_eq!(ErrorKind::from_status(400), ErrorKind::BadRequest);
    assert_eq!(ErrorKind::from_status(401), ErrorKind::Authentication);
    assert_eq!(ErrorKind::from_status(403), ErrorKind::PermissionDenied);
    assert_eq!(ErrorKind::from_status(404), ErrorKind::NotFound);
    assert_eq!(ErrorKind::from_status(409), ErrorKind::Conflict);
    assert_eq!(ErrorKind::from_status(429), ErrorKind::RateLimit);
    assert_eq!(ErrorKind::from_status(500), ErrorKind::Server);
    assert_eq!(ErrorKind::from_status(599), ErrorKind::Server);
    assert_eq!(ErrorKind::from_status(418), ErrorKind::Unknown);
}

#[test]
fn errorkind_display_strings() {
    assert_eq!(format!("{}", ErrorKind::BadRequest), "bad request");
    assert_eq!(format!("{}", ErrorKind::RateLimit), "rate limit");
    assert_eq!(format!("{}", ErrorKind::Transport), "transport error");
    assert_eq!(
        format!("{}", ErrorKind::Serialization),
        "serialization error"
    );
    assert_eq!(format!("{}", ErrorKind::Unknown), "unknown");
    assert_eq!(format!("{}", ErrorKind::Server), "server error");
    assert_eq!(format!("{}", ErrorKind::NotFound), "not found");
    assert_eq!(format!("{}", ErrorKind::Conflict), "conflict");
    assert_eq!(format!("{}", ErrorKind::Authentication), "authentication");
    assert_eq!(
        format!("{}", ErrorKind::PermissionDenied),
        "permission denied"
    );
}

#[test]
fn api_error_display_transport_kind() {
    // status_code == 0 -> the "kind:" variant of the Display impl.
    let e = ApiError::new(ErrorKind::Transport, "DNS failed");
    let s = format!("{}", e);
    assert!(s.contains("transport"));
    assert!(s.contains("DNS failed"));
}

#[test]
fn api_error_transport_constructor_chains_source() {
    let inner = std::io::Error::other("boom");
    let e = ApiError::transport("send failed", inner);
    assert_eq!(e.kind, ErrorKind::Transport);
    assert!(Error::source(&e).is_some());
}

#[test]
fn api_error_serialization_constructor_chains_source() {
    let inner = serde_json::from_str::<u8>("not json").unwrap_err();
    let e = ApiError::serialization("decode failed", inner);
    assert_eq!(e.kind, ErrorKind::Serialization);
    assert!(Error::source(&e).is_some());
}

#[test]
fn client_builder_trims_trailing_slashes() {
    let c = Client::builder()
        .base_url("https://example.com/")
        .api_key("k")
        .build()
        .unwrap();
    assert_eq!(c.base_url(), "https://example.com");
    assert_eq!(c.api_key().as_deref(), Some("k"));
}

#[test]
fn client_new_default() {
    let c = Client::new().unwrap();
    assert_eq!(c.base_url(), DEFAULT_BASE_URL);
    assert!(c.api_key().is_none());
}

#[test]
fn client_builder_user_agent_override() {
    let c = Client::builder().user_agent("custom/1.0").build().unwrap();
    // We can't directly observe the UA header without a mock, but the build
    // succeeded — and a separate test (`authenticated_request_sends_bearer`)
    // exercises the default path.
    let _ = c;
    // Sanity-check that the SDK version constant is populated.
    assert!(!SDK_VERSION.is_empty());
}
