//! Resource service handles — one per API tag.
//!
//! Build a [`Client`](crate::Client) and access these via the accessor
//! methods (`client.account()`, `client.numbers()`, …).

pub mod account;
pub mod acl;
pub mod authentication;
pub mod e911;
pub mod gateways;
pub mod inumbering;
pub mod lookups;
pub mod messaging;
pub mod numbers;
pub mod support;

pub use account::AccountService;
pub use acl::AclService;
pub use authentication::AuthenticationService;
pub use e911::E911Service;
pub use gateways::GatewaysService;
pub use inumbering::INumberingService;
pub use lookups::LookupsService;
pub use messaging::MessagingService;
pub use numbers::NumbersService;
pub use support::SupportService;
