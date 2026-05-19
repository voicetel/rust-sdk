//! Live read-only checks against api.voicetel.com.
//!
//! Gated behind `#[ignore]` and the VOICETEL_USERNAME / VOICETEL_PASSWORD
//! env vars. Run with:
//!
//! ```bash
//! export VOICETEL_USERNAME=...
//! export VOICETEL_PASSWORD=...
//! cargo test --test integration -- --ignored
//! ```
//!
//! Strict rules in this file:
//!  - No state mutations. No POST/PUT/PATCH/DELETE (except `Login` itself).
//!  - Login + account/registration + account/cdr + account/recurring-charges +
//!    account/payments all share a 6/hr/IP rate limit. Run sparingly.

use std::env;
use std::time::Duration;

use voicetel::{Client, DEFAULT_BASE_URL};

async fn integration_client() -> Client {
    let user = env::var("VOICETEL_USERNAME").ok();
    let pass = env::var("VOICETEL_PASSWORD").ok();
    let (user, pass) = match (user, pass) {
        (Some(u), Some(p)) => (u, p),
        _ => panic!("set VOICETEL_USERNAME / VOICETEL_PASSWORD to run integration tests"),
    };
    let base = env::var("VOICETEL_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    let uid: u64 = user.parse().expect("VOICETEL_USERNAME must be numeric");
    let client = Client::builder()
        .base_url(base)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();
    client.login(uid, &pass).await.expect("login");
    client
}

#[tokio::test]
#[ignore]
async fn account_get_live() {
    let c = integration_client().await;
    let me = c.account().get().await.expect("Account.get");
    assert!(me.username.is_some(), "Account.Get returned empty username");
}

#[tokio::test]
#[ignore]
async fn read_only_lists_live() {
    let c = integration_client().await;
    c.numbers().list().await.expect("Numbers.list");
    c.gateways().list().await.expect("Gateways.list");
    c.acl().list().await.expect("ACL.list");
    c.e911().list().await.expect("E911.list");
    c.support().list().await.expect("Support.list");
    c.i_numbering()
        .coverage(&Default::default())
        .await
        .expect("INumbering.coverage");
    c.i_numbering().ports().await.expect("INumbering.ports");
    c.authentication().get().await.expect("Authentication.get");
    c.messaging()
        .campaign_status()
        .await
        .expect("Messaging.campaign_status");
}
