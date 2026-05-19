//! `ACL` resource tests.

mod common;
use common::{envelope, test_client};

use voicetel::models::acl::*;
use voicetel::models::common::CidrEntry;
use voicetel::ErrorKind;
use wiremock::matchers::{method, path};
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
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "removed": [{"cidr": "203.0.113.5/32"}]
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
    let out = c.acl().remove(&body).await.unwrap();
    assert_eq!(out.removed[0].cidr, "203.0.113.5/32");
}

#[tokio::test]
async fn acl_conflict_409() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/acl"))
        .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
            "code": "conflict", "message": "partial"
        })))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let body = AclModifyRequest { acl: vec![] };
    let err = c.acl().add(&body).await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Conflict);
}
