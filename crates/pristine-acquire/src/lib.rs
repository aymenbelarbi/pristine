//! Pristine Acquire - Repository acquisition
//!
//! This crate handles acquiring repositories from various sources
//! including local directories and remote Git repositories.

pub mod git;
pub mod local;
pub mod github;

pub use git::GitAcquirer;
pub use local::LocalAcquirer;
pub use github::GitHubAcquirer;
