//! Pure unit tests — error construction, model serialisation quirks, builder defaults.

use std::error::Error;

use voicetel::models::account::AccountServices;
use voicetel::models::inumbering::{OrderNumber, PortAvailabilityData, PortSummary};
use voicetel::models::messaging::MessageSendRequest;
use voicetel::models::numbers::{
    CampaignAssignStatus, MessagingEnablementJob, MessagingEnablementJobState,
    NumberMessagingCampaignAssignData, NumberMoveData, OrderId, PortOutPinUpdateData,
};
use voicetel::models::support::SupportConversation;
use voicetel::{ApiError, Client, ErrorKind, DEFAULT_BASE_URL, SDK_VERSION};

#[test]
fn ordernumber_plain_serialises_as_string() {
    let n = OrderNumber::plain("2015550123");
    let v = serde_json::to_value(&n).unwrap();
    assert_eq!(v, serde_json::json!("2015550123"));
}

#[test]
fn ordernumber_with_route_serialises_as_object() {
    let n = OrderNumber::with_route("2015550123", 5);
    let v = serde_json::to_value(&n).unwrap();
    assert_eq!(v, serde_json::json!({"number": "2015550123", "route": 5}));
}

#[test]
fn ordernumber_deserialises_string() {
    let n: OrderNumber = serde_json::from_str("\"2015550123\"").unwrap();
    assert_eq!(n, OrderNumber::Plain("2015550123".into()));
}

#[test]
fn ordernumber_deserialises_object() {
    let n: OrderNumber =
        serde_json::from_str("{\"number\": \"2015550123\", \"route\": 4}").unwrap();
    match n {
        OrderNumber::WithRoute { number, route } => {
            assert_eq!(number, "2015550123");
            assert_eq!(route, Some(4));
        }
        _ => panic!("expected WithRoute"),
    }
}

#[test]
fn ordernumber_deserialises_object_without_route() {
    let n: OrderNumber = serde_json::from_str("{\"number\": \"2015550123\"}").unwrap();
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
        from_number: "2015550123".into(),
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
        number: "2015550123".into(),
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
        "number": "2015550123",
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
fn campaign_assign_status_wire_values() {
    assert_eq!(
        serde_json::to_value(CampaignAssignStatus::Enabled).unwrap(),
        serde_json::json!("enabled")
    );
    assert_eq!(
        serde_json::to_value(CampaignAssignStatus::Already).unwrap(),
        serde_json::json!("already")
    );
    assert_eq!(
        serde_json::to_value(CampaignAssignStatus::Pending).unwrap(),
        serde_json::json!("pending")
    );
    let s: CampaignAssignStatus = serde_json::from_str("\"already\"").unwrap();
    assert_eq!(s, CampaignAssignStatus::Already);
    assert!(serde_json::from_str::<CampaignAssignStatus>("\"nope\"").is_err());
}

#[test]
fn messaging_enablement_job_state_wire_values() {
    for (wire, variant) in [
        ("queued", MessagingEnablementJobState::Queued),
        ("hosting", MessagingEnablementJobState::Hosting),
        ("assigning", MessagingEnablementJobState::Assigning),
        ("routing", MessagingEnablementJobState::Routing),
        ("done", MessagingEnablementJobState::Done),
        ("failed", MessagingEnablementJobState::Failed),
    ] {
        assert_eq!(
            serde_json::to_value(variant).unwrap(),
            serde_json::json!(wire)
        );
        let back: MessagingEnablementJobState =
            serde_json::from_str(&format!("\"{}\"", wire)).unwrap();
        assert_eq!(back, variant);
    }
    assert!(serde_json::from_str::<MessagingEnablementJobState>("\"gone\"").is_err());
}

#[test]
fn messaging_enablement_job_uses_camel_case_wire_names() {
    let raw = r#"{
        "campaignId": "CAMP002",
        "state": "failed",
        "message": "carrier rejected the hosting request",
        "createdAt": "2026-10-01T12:00:00Z",
        "updatedAt": "2026-10-01T12:05:00Z"
    }"#;
    let job: MessagingEnablementJob = serde_json::from_str(raw).unwrap();
    assert_eq!(job.campaign_id, "CAMP002");
    assert_eq!(job.state, MessagingEnablementJobState::Failed);
    let v = serde_json::to_value(&job).unwrap();
    assert!(v.get("campaignId").is_some());
    assert!(v.get("createdAt").is_some());
    assert!(v.get("updatedAt").is_some());
    assert!(v.get("campaign_id").is_none());
}

#[test]
fn order_id_accepts_string_or_number() {
    let s: OrderId = serde_json::from_str("\"ORD-1\"").unwrap();
    assert_eq!(s, OrderId::Text("ORD-1".into()));
    let n: OrderId = serde_json::from_str("987654").unwrap();
    assert_eq!(n, OrderId::Numeric(987654));
    assert_eq!(serde_json::to_value(&n).unwrap(), serde_json::json!(987654));
}

#[test]
fn assign_data_new_v2_2_14_fields_round_trip() {
    let raw = r#"{
        "number": "2015550123",
        "campaignId": "CAMP002",
        "status": "enabled",
        "carrier": 19,
        "network": "B",
        "upstreamCnpId": "CAMP003",
        "previousNetwork": null,
        "previousNetworkCleared": false,
        "alreadyAssigned": false,
        "orderId": "ORD-1",
        "warning": null,
        "message": null,
        "job": null
    }"#;
    let d: NumberMessagingCampaignAssignData = serde_json::from_str(raw).unwrap();
    assert_eq!(d.status, CampaignAssignStatus::Enabled);
    assert!(!d.already_assigned);
    assert_eq!(d.order_id, Some(OrderId::Text("ORD-1".into())));
    assert!(d.warning.is_none());
    assert!(d.job.is_none());
    let v = serde_json::to_value(&d).unwrap();
    assert_eq!(v["status"], serde_json::json!("enabled"));
    assert!(v.get("alreadyAssigned").is_some());
    assert!(v.get("orderId").is_some());
    assert!(v.get("already_assigned").is_none());
}

#[test]
fn port_out_pin_update_data_reports_changed() {
    let raw = r#"{"number": "2015550123", "portOutPin": "1234", "changed": false}"#;
    let d: PortOutPinUpdateData = serde_json::from_str(raw).unwrap();
    assert_eq!(d.port_out_pin, "1234");
    assert!(!d.changed);
    let v = serde_json::to_value(&d).unwrap();
    assert_eq!(v["portOutPin"], serde_json::json!("1234"));
    assert_eq!(v["changed"], serde_json::json!(false));
}

#[test]
fn account_services_no_longer_has_rcid() {
    let raw = r#"{"sms": true, "mms": false, "rcid": true}"#;
    let s: AccountServices = serde_json::from_str(raw).unwrap();
    assert_eq!(s.sms, Some(true));
    let v = serde_json::to_value(&s).unwrap();
    assert!(v.get("rcid").is_none());
}

#[test]
fn port_summary_no_longer_has_support_url() {
    let raw = r#"{"status": "active", "id": "1", "supportUrl": "https://x"}"#;
    let p: PortSummary = serde_json::from_str(raw).unwrap();
    assert_eq!(p.status, "active");
    let v = serde_json::to_value(&p).unwrap();
    assert!(v.get("supportUrl").is_none());
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
