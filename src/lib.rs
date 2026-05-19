//! # 📞 VoiceTel Rust SDK
//!
//! The official Rust client for the [VoiceTel REST API](https://voicetel.com/docs/api/v2.2/) —
//! provision numbers, place orders, validate e911, send messages, and manage
//! your account, with strongly-typed `serde`-derived models and async-first
//! ergonomics on top of [`reqwest`].
//!
//! ## Quickstart
//!
//! ```no_run
//! use std::time::Duration;
//! use voicetel::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::builder()
//!         .timeout(Duration::from_secs(30))
//!         .max_retries(2)
//!         .build()?;
//!
//!     // Exchange username + password for an API key (one-time per session).
//!     client.login(1_000_000_001, "hunter2").await?;
//!
//!     // Typed responses — your IDE knows what `me` is.
//!     let me = client.account().get().await?;
//!     println!("Balance: ${:.2}", me.cash.unwrap_or_default());
//!
//!     // List the account's numbers.
//!     let numbers = client.numbers().list().await?;
//!     for n in numbers.numbers {
//!         println!("{} route={}", n.number, n.route);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! Every endpoint requires `Authorization: Bearer <apikey>` except
//! `POST /v2.2/account/api-key`, which exchanges `{username, password}` for
//! a fresh 32-hex key. [`Client::login`] handles the exchange and installs
//! the key on the transport. If you already have a key, pass it via
//! [`ClientBuilder::api_key`].
//!
//! ## Error handling
//!
//! Every fallible method returns [`Result<T, ApiError>`](ApiError). Inspect
//! [`ApiError::kind`] to branch on the failure mode:
//!
//! ```no_run
//! # use voicetel::{Client, ErrorKind};
//! # async fn run(client: Client) -> Result<(), Box<dyn std::error::Error>> {
//! match client.numbers().get("9999999999").await {
//!     Ok(n) => println!("{:?}", n),
//!     Err(e) if e.is_not_found() => eprintln!("not on this account"),
//!     Err(e) if e.kind == ErrorKind::RateLimit => eprintln!("slow down"),
//!     Err(e) => return Err(e.into()),
//! }
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod models;
pub mod resources;
pub mod transport;
pub mod version;

mod client;

pub use crate::client::{Client, ClientBuilder};
pub use crate::error::{ApiError, ErrorKind};
pub use crate::version::{API_VERSION, DEFAULT_BASE_URL, DEFAULT_USER_AGENT, SDK_VERSION};
