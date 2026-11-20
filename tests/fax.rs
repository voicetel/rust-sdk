//! `Fax` resource tests — wire shapes including the multipart send body and
//! the binary document download.

mod common;
use common::{envelope, test_client};

use voicetel::models::fax::*;
use voicetel::ErrorKind;
use wiremock::matchers::{header, header_regex, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A canonical `FaxJob` payload (spec example) reused by the history and
/// single-job tests.
fn job_json() -> serde_json::Value {
    serde_json::json!({
        "id": "9f8c2b41a",
        "fromNumber": "2015550123",
        "toNumber": "2125550123",
        "clientToken": "123e4567-e89b-12d3-a456-426614174000",
        "status": "sent",
        "submittedAt": "2026-11-20T15:04:05Z",
        "startedAt": "2026-11-20T15:04:41Z",
        "completedAt": "2026-11-20T15:05:19Z",
        "attempts": 1,
        "failureReason": null,
        "documents": [
            {"name": "contract.pdf", "size": 123456, "contentType": "application/pdf"}
        ],
        "documentAvailable": true,
        "documentName": "contract.pdf",
        "documentSize": 123456
    })
}

#[tokio::test]
async fn fax_numbers_lists_capable_numbers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/numbers/fax"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "numbers": [
                    {"number": "2015550123", "destinationId": 7, "email": "fax@example.com"},
                    {"number": "2125550123", "destinationId": 0, "email": "acct@example.com"}
                ]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c.fax().numbers().await.unwrap();
    assert_eq!(out.numbers.len(), 2);
    assert_eq!(out.numbers[0].number, "2015550123");
    assert_eq!(out.numbers[0].destination_id, 7);
    assert_eq!(out.numbers[0].email, "fax@example.com");
    assert_eq!(out.numbers[1].destination_id, 0);
}

#[tokio::test]
async fn fax_send_posts_multipart_with_uuid_idempotency_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/faxes"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=.+",
        ))
        .and(header_regex(
            "idempotency-key",
            "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
        ))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(envelope(serde_json::json!({
                "id": "9f8c2b41a",
                "fromNumber": "2015550123",
                "toNumber": "2125550123",
                "status": "queued",
                "documents": [
                    {"name": "contract.pdf", "size": 16, "contentType": "application/pdf"}
                ]
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let req = FaxSendRequest {
        from_number: "2015550123".into(),
        to_number: "2125550123".into(),
        documents: vec![FaxDocument {
            name: "contract.pdf".into(),
            content_type: "application/pdf".into(),
            data: b"%PDF-1.4 fake".to_vec(),
        }],
        idempotency_key: None,
    };
    let out = c.fax().send(&req).await.unwrap();
    assert_eq!(out.id, "9f8c2b41a");
    assert_eq!(out.from_number, "2015550123");
    assert_eq!(out.to_number, "2125550123");
    assert_eq!(out.status, "queued");
    assert_eq!(out.documents.len(), 1);
    assert_eq!(out.documents[0].name, "contract.pdf");
    assert_eq!(out.documents[0].size, 16);
    assert_eq!(out.documents[0].content_type, "application/pdf");

    // Wire shape: multipart body with both text fields and the file part,
    // delimited by the boundary advertised in the Content-Type header.
    let requests = server.received_requests().await.expect("recorded requests");
    let sent = requests.last().expect("one POST request");
    let boundary = sent.headers["content-type"]
        .to_str()
        .unwrap()
        .split("boundary=")
        .nth(1)
        .expect("boundary parameter");
    let body = String::from_utf8_lossy(&sent.body).into_owned();
    let expected = format!(
        "--{b}\r\n\
         Content-Disposition: form-data; name=\"fromNumber\"\r\n\
         \r\n\
         2015550123\r\n\
         --{b}\r\n\
         Content-Disposition: form-data; name=\"toNumber\"\r\n\
         \r\n\
         2125550123\r\n\
         --{b}\r\n\
         Content-Disposition: form-data; name=\"documents\"; filename=\"contract.pdf\"\r\n\
         Content-Type: application/pdf\r\n\
         \r\n\
         %PDF-1.4 fake\r\n\
         --{b}--\r\n",
        b = boundary
    );
    assert_eq!(body, expected);
}

#[tokio::test]
async fn fax_send_honors_explicit_idempotency_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/faxes"))
        .and(header(
            "idempotency-key",
            "123e4567-e89b-12d3-a456-426614174000",
        ))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(envelope(serde_json::json!({
                "id": "9f8c2b41a",
                "fromNumber": "2015550123",
                "toNumber": "2125550123",
                "status": "queued",
                "documents": []
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let req = FaxSendRequest {
        from_number: "2015550123".into(),
        to_number: "2125550123".into(),
        documents: vec![FaxDocument {
            name: "contract.pdf".into(),
            content_type: "application/pdf".into(),
            data: b"%PDF-1.4 fake".to_vec(),
        }],
        idempotency_key: Some("123e4567-e89b-12d3-a456-426614174000".into()),
    };
    let out = c.fax().send(&req).await.unwrap();
    assert_eq!(out.id, "9f8c2b41a");
}

#[tokio::test]
async fn fax_history_sends_query_parameters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/faxes"))
        .and(query_param("number", "2015550123"))
        .and(query_param("from", "1795100000"))
        .and(query_param("to", "1795013600"))
        .and(query_param("limit", "25"))
        .and(query_param("cursor", "cGl0LnN0b3Jl.8b1c0f2a"))
        .and(query_param("status", "failed"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "number": "2015550123",
                "fromTs": 1795100000,
                "toTs": 1795013600,
                "faxes": [job_json()],
                "nextCursor": null
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let out = c
        .fax()
        .history(&FaxHistoryOptions {
            number: "2015550123".into(),
            from: Some(1_795_100_000),
            to: Some(1_795_013_600),
            limit: Some(25),
            cursor: Some("cGl0LnN0b3Jl.8b1c0f2a".into()),
            status: Some(FaxHistoryStatus::Failed),
        })
        .await
        .unwrap();
    assert_eq!(out.number, "2015550123");
    assert_eq!(out.from_ts, 1_795_100_000);
    assert_eq!(out.to_ts, 1_795_013_600);
    assert_eq!(out.faxes.len(), 1);
    assert_eq!(out.faxes[0].id, "9f8c2b41a");
    assert_eq!(out.faxes[0].status, FaxJobStatus::Sent);
    assert!(out.next_cursor.is_none());
}

#[tokio::test]
async fn fax_get_decodes_full_job() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/faxes/9f8c2b41a"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(job_json())))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let job = c.fax().get("9f8c2b41a").await.unwrap();
    assert_eq!(job.id, "9f8c2b41a");
    assert_eq!(job.from_number, "2015550123");
    assert_eq!(job.to_number, "2125550123");
    assert_eq!(
        job.client_token.as_deref(),
        Some("123e4567-e89b-12d3-a456-426614174000")
    );
    assert_eq!(job.status, FaxJobStatus::Sent);
    assert_eq!(job.submitted_at, "2026-11-20T15:04:05Z");
    assert_eq!(job.started_at.as_deref(), Some("2026-11-20T15:04:41Z"));
    assert_eq!(job.completed_at.as_deref(), Some("2026-11-20T15:05:19Z"));
    assert_eq!(job.attempts, Some(1));
    assert!(job.failure_reason.is_none());
    assert_eq!(job.documents[0].size, 123456);
    assert!(job.document_available);
    assert_eq!(job.document_name.as_deref(), Some("contract.pdf"));
    assert_eq!(job.document_size, Some(123456));
}

#[tokio::test]
async fn fax_get_decodes_active_job_with_null_lifecycle_fields() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/faxes/abc123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
                "id": "abc123",
                "fromNumber": "2015550123",
                "toNumber": "2125550123",
                "clientToken": null,
                "status": "retry_pending",
                "submittedAt": "2026-11-20T15:04:05Z",
                "startedAt": "2026-11-20T15:04:41Z",
                "completedAt": null,
                "attempts": null,
                "failureReason": null,
                "documents": [],
                "documentAvailable": false,
                "documentName": null,
                "documentSize": null
            }))),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let job = c.fax().get("abc123").await.unwrap();
    assert_eq!(job.status, FaxJobStatus::RetryPending);
    assert!(job.client_token.is_none());
    assert!(job.completed_at.is_none());
    assert!(job.attempts.is_none());
    assert!(job.documents.is_empty());
    assert!(!job.document_available);
    assert!(job.document_name.is_none());
    assert!(job.document_size.is_none());
}

#[tokio::test]
async fn fax_document_returns_raw_pdf_bytes() {
    let server = MockServer::start().await;
    let pdf = b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\nbinary fax body \x00\x01".to_vec();
    Mock::given(method("GET"))
        .and(path("/v2.2/faxes/9f8c2b41a/document"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/pdf")
                .insert_header(
                    "content-disposition",
                    "attachment; filename=\"contract.pdf\"",
                )
                .set_body_bytes(pdf.clone()),
        )
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let bytes = c.fax().document("9f8c2b41a").await.unwrap();
    assert_eq!(bytes, pdf);
}

#[tokio::test]
async fn fax_document_404_maps_to_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2.2/faxes/9f8c2b41a/document"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "status": "error",
            "error": {"code": 404, "message": "Not found"},
            "serverTime": 1795100000
        })))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let err = c.fax().document("9f8c2b41a").await.unwrap_err();
    assert_eq!(err.kind, ErrorKind::NotFound);
    assert_eq!(err.status_code, 404);
    assert_eq!(err.code, Some(404));
    assert_eq!(err.message, "Not found");
}

#[tokio::test]
async fn fax_error_envelope_with_servertime_is_parsed_tolerantly() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2.2/faxes"))
        .respond_with(ResponseTemplate::new(402).set_body_json(serde_json::json!({
            "status": "error",
            "error": {"code": 402, "message": "Your account needs a positive balance to send faxes."},
            "serverTime": 1795100000
        })))
        .mount(&server)
        .await;
    let c = test_client(&server).await;
    let req = FaxSendRequest {
        from_number: "2015550123".into(),
        to_number: "2125550123".into(),
        documents: vec![FaxDocument {
            name: "contract.pdf".into(),
            content_type: "application/pdf".into(),
            data: b"%PDF-1.4 fake".to_vec(),
        }],
        idempotency_key: None,
    };
    let err = c.fax().send(&req).await.unwrap_err();
    assert_eq!(err.status_code, 402);
    assert_eq!(err.code, Some(402));
    assert_eq!(
        err.message,
        "Your account needs a positive balance to send faxes."
    );
}
