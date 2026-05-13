//! Pristine Inventory - File system traversal
//!
//! This crate handles walking the file system and creating
//! a catalog of files in a repository.

pub mod walker;
pub mod ignore;

pub use walker::FileWalker;
pub use ignore::IgnoreRules;
