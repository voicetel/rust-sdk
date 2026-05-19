//! Strongly-typed request and response models for every operation.
//!
//! All models derive [`serde::Serialize`] + [`serde::Deserialize`]; optional
//! fields are `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
//! so PATCH-style partial updates never send a field you didn't set.

pub mod account;
pub mod acl;
pub mod authentication;
pub mod common;
pub mod e911;
pub mod gateways;
pub mod inumbering;
pub mod lookups;
pub mod messaging;
pub mod numbers;
pub mod support;

pub use common::CidrEntry;
