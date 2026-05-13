//! Cache configuration

use serde::{Deserialize, Serialize};

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache directory
    pub directory: String,
    /// Maximum snapshot entries
    pub max_snapshot_entries: u64,
    /// Snapshot cache TTL in seconds
    pub snapshot_ttl_seconds: u64,
    /// Maximum inventory entries
    pub max_inventory_entries: u64,
    /// Inventory cache TTL in seconds
    pub inventory_ttl_seconds: u64,
    /// Maximum artifact entries
    pub max_artifact_entries: u64,
    /// Artifact cache TTL in seconds
    pub artifact_ttl_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: ".pristine/cache".to_string(),
            max_snapshot_entries: 100,
            snapshot_ttl_seconds: 86_400,    // 24 hours
            max_inventory_entries: 100,
            inventory_ttl_seconds: 43_200,   // 12 hours
            max_artifact_entries: 1000,
            artifact_ttl_seconds: 3_600,     // 1 hour
        }
    }
}
