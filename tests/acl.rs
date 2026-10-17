//! `ACL` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::acl::*;
use voicetel::models::common::CidrEntry;
use voicetel::ErrorKind;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn acl_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/acl"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "acl": [{"cidr": "192.0.2.0/24"}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.acl().list().await.unwrap();
    assert_eq!(out.acl.len(), 1);
}

#[tokio::test]
async fn acl_add() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/acl"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "added": [{"cidr": "203.0.113.5/32"}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AclModifyRequest {
        acl: vec![CidrEntry {
            cidr: "203.0.113.5/32".into(),
        }],
    };
    let out = c.acl().add(&body).await.unwrap();
    assert_eq!(out.added.len(), 1);
}

#[tokio::test]
async fn acl_remove() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v2.2/acl"))
        .and(query_param("cidrs", "203.0.113.5/32,203.0.113.9/32"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "removed": [{"cidr": "203.0.113.5/32"}, {"cidr": "203.0.113.9/32"}]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .acl()
        .remove(&["203.0.113.5/32".into(), "203.0.113.9/32".into()])
        .await
        .unwrap();
    assert_eq!(out.removed[0].cidr, "203.0.113.5/32");
    assert_eq!(out.removed.len(), 2);
}

#[tokio::test]
async fn acl_conflict_409() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/acl"))
        .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
            "status": "error",
            "error": {"code": 409, "message": "CIDR overlaps with existing entry"},
            "data": {
                "added": [{"cidr": "203.0.113.0/24"}],
                "failed": [{"cidr": "abc123", "reason": "DB Insert failed"}]
            }
        })))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AclModifyRequest {
        acl: vec![CidrEntry {
            cidr: "203.0.113.0/24".into(),
        }],
    };
    let err = c.acl().add(&body).await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Conflict);
    assert_eq!(err.code, Some(409));
    assert_eq!(err.message, "CIDR overlaps with existing entry");
    // The structured 409 detail block is available as a typed model.
    let detail = err.acl_conflict().expect("typed conflict data");
    assert_eq!(detail.added.as_deref().map(|a| a.len()), Some(1));
    let failed = detail.failed.as_deref().expect("failed entries");
    assert_eq!(failed[0].cidr, "abc123");
    assert_eq!(failed[0].reason, "DB Insert failed");
}
