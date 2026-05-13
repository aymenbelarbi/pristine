//! Pristine Cache - Caching layer
//!
//! This crate provides caching for snapshots, inventories,
//! and artifacts using an in-memory LRU cache.

pub mod cache;
pub mod config;

pub use cache::CacheManager;
pub use config::CacheConfig;
