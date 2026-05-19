# 📞 VoiceTel Rust SDK

The official Rust client for the [VoiceTel REST API](https://voicetel.com/docs/api/v2.2/) — provision numbers, place orders, validate e911, send messages, and manage your account, all with strongly-typed `serde` models and an async-first ergonomics on top of [`reqwest`].

![Version](https://img.shields.io/badge/version-2.2.10-blue)
![Rust](https://img.shields.io/badge/rust-1.75%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Coverage](https://img.shields.io/badge/coverage-93%25-brightgreen)
[![docs.rs](https://img.shields.io/docsrs/voicetel)](https://docs.rs/voicetel)

## 📚 Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Quickstart](#-quickstart)
- [Authentication](#-authentication)
- [Resource Reference](#-resource-reference)
- [Error Handling](#-error-handling)
- [Cancellation and Timeouts](#-cancellation-and-timeouts)
- [Rate Limits](#-rate-limits)
- [Development](#-development)
- [API Documentation](#-api-documentation)
- [Contributors](#-contributors)
- [Sponsors](#-sponsors)
- [License](#-license)

## ✨ Features

### 🛡️ Strongly Typed End-to-End
- **`#[derive(Serialize, Deserialize)]` on every model** — 73 operations across 10 resources, no `serde_json::Value` smuggling.
- **`Option<T>` + `skip_serializing_if`** on optional request fields — distinguish "not set" from "zero" cleanly when PATCH-ing.
- **`Option<T>` for nullable response fields** — `forward_to: Option<String>` lets you tell apart "no forward configured" from an empty destination.
- **Wire-name aware** — `fromNumber`, `localRoutingNumber`, `rateCenterTier`, `ticketNumber` and friends are all surfaced under idiomatic snake_case Rust names via `#[serde(rename = ...)]`.

### 🔁 Production-Grade Transport
- Built on **`reqwest`** with `rustls-tls`, `json`, and `gzip` features — no OpenSSL surprises.
- **Automatic retry** with exponential backoff on 429 / 5xx — honours `Retry-After` headers, capped at 8s. Configurable via `Client::builder().max_retries(n)`.
- **Configurable timeout** per client (defaults to 30s).
- **Bearer auth** managed for you; the password→key exchange is one method call (`Client::login`).
- **Structured `ApiError`** with typed `kind: ErrorKind` so you can `match err.kind { ErrorKind::RateLimit => ... }` without parsing HTTP status codes.
- **Envelope unwrap** — `{"status":"success","data": ...}` is stripped before the response hits your code.

### 📞 Complete API Coverage
- **Numbers** — list, get, add, remove, route, translate, CNAM, LIDB, fax, forward, SMS, messaging campaigns, port-out PIN, account moves.
- **Account** — profile, sub-accounts, CDRs, credits, payments, MRC, registration, password recovery.
- **e911** — record provisioning, address validation, lookup, removal.
- **Gateways** — list, create, update, delete, view bound numbers.
- **Messaging** — SMS & MMS sending, message history, 10DLC brand and campaign registration, per-number messaging state.
- **Lookups** — CNAM and LRN dips.
- **iNumbering** — inventory search, coverage queries, number orders, port-in submissions, port-out availability (with v2.2.10 `localRoutingNumber` / `rateCenterTier`).
- **Support** — ticket create / read / update / delete, threaded messages, replies.
- **ACL** — IP allowlist management with structured 409 conflict bodies (`AclConflictData`).
- **Authentication** — switch between Digest, IP-only, or hybrid modes; rotate passwords.

### 🧪 Battle-Tested
- **77 unit tests** + 2 doc-tests + 2 integration tests via **`wiremock`**.
- **93% line coverage** (cargo-tarpaulin, llvm engine).
- Every method's happy path and error path covered.
- **`cargo fmt --check`** clean.
- **`cargo clippy --all-targets -- -D warnings`** clean.

### 📦 Clean Distribution
- Single crate (`voicetel`); install with `cargo add voicetel`.
- Minimal dependency surface: `reqwest`, `serde`, `serde_json`, `tokio` (time only), `thiserror`, `url`.

## 🚀 Installation

```toml
[dependencies]
voicetel = "2.2.10"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Or with `cargo add`:

```bash
cargo add voicetel
cargo add tokio --features macros,rt-multi-thread
```

Requires Rust **1.75** or later.

## 🏁 Quickstart

```rust
use std::time::Duration;
use voicetel::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .max_retries(2)
        .build()?;

    // Exchange username + password for an API key (one-time per session)
    client.login(1_000_000_001, "hunter2").await?;

    // Typed responses — your IDE knows what `me` is.
    let me = client.account().get().await?;
    println!("Balance: ${:.2}", me.cash.unwrap_or_default());

    // List your numbers
    let numbers = client.numbers().list().await?;
    for n in numbers.numbers {
        println!("{} route={} cnam={} sms={}", n.number, n.route, n.cnam, n.sms_enabled);
    }
    Ok(())
}
```

Or, if you already have an API key:

```rust
use voicetel::{Client, models::inumbering::CoverageQuery};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder().api_key("32hex...").build()?;
let q = CoverageQuery { state: Some("NJ".into()), ..Default::default() };
let coverage = client.i_numbering().coverage(&q).await?;
for bucket in coverage.coverage {
    println!("{:?}-{:?}: {} TNs", bucket.npa, bucket.nxx, bucket.count);
}
# Ok(()) }
```

## 🔑 Authentication

Every endpoint requires `Authorization: Bearer <apikey>` **except** `POST /v2.2/account/api-key`, which exchanges username + password for a fresh 32-hex key. `Client::login()` handles the exchange and installs the returned key on the transport.

Re-fetch the API key after any password change — the old one is invalidated.

> Don't have credentials yet? Get them at **[voicetel.com/docs/api/v2.2/credentials](https://voicetel.com/docs/api/v2.2/credentials/)**.

```rust
# use voicetel::Client;
# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder().build()?;
let key = client.login(1_000_000_001, "hunter2").await?;
// `key` is the new 32-hex bearer; the client already has it installed.
# Ok(()) }
```

## 🗺️ Resource Reference

| Resource | Method on `Client` | Example |
|---|---|---|
| Account | `client.account()` | `client.account().cdr(t1, t2).await?` |
| ACL | `client.acl()` | `client.acl().add(&body).await?` |
| Authentication | `client.authentication()` | `client.authentication().update(&body).await?` |
| e911 | `client.e911()` | `client.e911().validate(&body).await?` |
| Gateways | `client.gateways()` | `client.gateways().list().await?` |
| iNumbering | `client.i_numbering()` | `client.i_numbering().search_inventory(&q).await?` |
| Lookups | `client.lookups()` | `client.lookups().lrn("2015551234", "2012548000").await?` |
| Messaging | `client.messaging()` | `client.messaging().send(&body).await?` |
| Numbers | `client.numbers()` | `client.numbers().assign_campaign("2015551234", &body).await?` |
| Support | `client.support()` | `client.support().create(&body).await?` |

Optional request fields are `Option<T>` — use `Some(...)` or the `..Default::default()` syntax to leave them unset:

```rust
use voicetel::models::account::AccountPutRequest;

let body = AccountPutRequest {
    timezone: Some("America/Chicago".into()),
    notify_threshold: Some(5),
    notify: Some(true),
    ..Default::default()
};
```

## 🚨 Error Handling

All HTTP errors return an `ApiError`. Inspect `kind` or use the boolean helpers:

| `ErrorKind` | HTTP status |
|---|---|
| `BadRequest` | 400 |
| `Authentication` | 401 |
| `PermissionDenied` | 403 |
| `NotFound` | 404 |
| `Conflict` | 409 |
| `RateLimit` | 429 |
| `Server` | 5xx |
| `Transport` | transport-level (DNS/TLS/timeout) |
| `Serialization` | JSON decode failure |
| `Unknown` | anything else |

```rust
# use voicetel::{Client, ErrorKind};
# async fn run(client: Client) -> Result<(), Box<dyn std::error::Error>> {
match client.numbers().get("9999999999").await {
    Ok(n) => println!("{:?}", n),
    Err(e) if e.is_not_found() => eprintln!("not on this account"),
    Err(e) if e.kind == ErrorKind::RateLimit => eprintln!("slow down"),
    Err(e) => return Err(e.into()),
}
# Ok(()) }
```

## ⏱️ Cancellation and Timeouts

Cancellation works through Tokio's normal mechanisms — wrap the call in `tokio::time::timeout(...)` for a per-call deadline:

```rust
use std::time::Duration;
use tokio::time::timeout;

# use voicetel::Client;
# async fn run(client: Client) -> Result<(), Box<dyn std::error::Error>> {
let me = timeout(Duration::from_secs(5), client.account().get()).await??;
# Ok(()) }
```

For a global per-request timeout, set it on the client:

```rust
# use std::time::Duration;
# use voicetel::Client;
# fn run() -> Result<Client, Box<dyn std::error::Error>> {
let client = Client::builder()
    .timeout(Duration::from_secs(45))
    .build()?;
# Ok(client) }
```

Or supply a fully-configured `reqwest::Client` (e.g. with a connection pool, proxy, or instrumentation):

```rust
# use std::time::Duration;
# use voicetel::Client;
# fn run() -> Result<Client, Box<dyn std::error::Error>> {
let http = reqwest::Client::builder()
    .timeout(Duration::from_secs(45))
    .pool_max_idle_per_host(10)
    .build()?;
let client = Client::builder().http_client(http).build()?;
# Ok(client) }
```

## ⏱️ Rate Limits

These endpoints are limited to **6 requests per hour per IP**:

- `account/info` (`AccountService::get`)
- `account/cdr` (`AccountService::cdr`)
- `account/recurring-charges` (`AccountService::recurring_charges`)
- `account/payments` (`AccountService::payments`)
- `account/registration` (`AccountService::registration`)
- `account/api-key` (`Client::login`)

The SDK automatically retries 429 responses with `Retry-After` honoured, up to `max_retries(n)` (default 2):

```rust
# use std::time::Duration;
# use voicetel::Client;
# fn run() -> Result<Client, Box<dyn std::error::Error>> {
let client = Client::builder()
    .api_key("32hex...")
    .max_retries(4)
    .timeout(Duration::from_secs(60))
    .build()?;
# Ok(client) }
```

## 🛠️ Development

```bash
git clone https://github.com/voicetel/rust-sdk
cd rust-sdk

# Run unit tests
cargo test

# Lint
cargo fmt --check
cargo clippy --all-targets -- -D warnings

# Build docs
cargo doc --no-deps --open

# Coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Xml

# Integration tests (live, read-only — needs credentials)
export VOICETEL_USERNAME=...
export VOICETEL_PASSWORD=...
cargo test --test integration -- --ignored
```

## 📖 API Documentation

- **Reference docs:** [voicetel.com/docs/api/v2.2/](https://voicetel.com/docs/api/v2.2/)
- **Interactive playground:** [voicetel.com/docs/api/v2.2/playground/](https://voicetel.com/docs/api/v2.2/playground/) — try the API in your browser without writing any code
- **API credentials:** [voicetel.com/docs/api/v2.2/credentials/](https://voicetel.com/docs/api/v2.2/credentials/)
- **Crate docs:** [docs.rs/voicetel](https://docs.rs/voicetel)

## 🙌 Contributors

- [Michael Mavroudis](https://github.com/mavroudis) — Lead Developer

Contributions welcome. Open an issue describing the change, or send a pull request against `main`.

## 💖 Sponsors

| Sponsor | Contribution |
|---------|--------------|
| [VoiceTel Communications](https://voicetel.com) | Primary development and production hosting |

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
