//! Pristine Safety - Secret scanning and policy enforcement
//!
//! This crate handles secret detection and policy enforcement
//! to ensure safe code sharing.

pub mod scanner;
pub mod policy;
pub mod patterns;

pub use scanner::SecretScanner;
pub use policy::PolicyEnforcer;
