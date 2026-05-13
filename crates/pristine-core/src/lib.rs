//! Pristine Core - Domain types and traits
//!
//! This crate contains the fundamental types and traits used throughout
//! the Pristine codebase. It has no external dependencies beyond the
//! workspace's core libraries.

pub mod types;
pub mod error;
pub mod traits;

pub use types::*;
pub use error::*;
pub use traits::*;
