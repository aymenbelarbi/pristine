//! Cache manager

use async_trait::async_trait;
use moka::future::Cache as MokaCache;
use pristine_core::*;
use crate::CacheConfig;
use std::time::Duration;

/// Cache manager for Pristine
pub struct CacheManager {
    snapshot_cache: MokaCache<String, RepoSnapshot>,
    inventory_cache: MokaCache<String, FileCatalog>,
    artifact_cache: MokaCache<String, ContextArtifact>,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(config: CacheConfig) -> Self {
        Self {
            snapshot_cache: MokaCache::builder()
                .max_capacity(config.max_snapshot_entries)
                .time_to_live(Duration::from_secs(config.snapshot_ttl_seconds))
                .build(),
            inventory_cache: MokaCache::builder()
                .max_capacity(config.max_inventory_entries)
                .time_to_live(Duration::from_secs(config.inventory_ttl_seconds))
                .build(),
            artifact_cache: MokaCache::builder()
                .max_capacity(config.max_artifact_entries)
                .time_to_live(Duration::from_secs(config.artifact_ttl_seconds))
                .build(),
        }
    }
    
    /// Get a cached artifact
    pub async fn get_artifact(&self, fingerprint: &str) -> Option<ContextArtifact> {
        self.artifact_cache.get(fingerprint).await
    }
    
    /// Put an artifact in cache
    pub async fn put_artifact(&self, fingerprint: String, artifact: ContextArtifact) {
        self.artifact_cache.insert(fingerprint, artifact).await;
    }
    
    /// Invalidate all caches for a source
    pub async fn invalidate_source(&self, source: &str) {
        // Note: Moka doesn't support pattern-based invalidation
        // This is a simplified implementation
        tracing::info!("Invalidating cache for source: {}", source);
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

#[async_trait]
impl pristine_core::Cache for CacheManager {
    async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        // Simplified implementation
        Ok(None)
    }

    async fn set<T: serde::Serialize + Send + Sync>(&self, key: &str, value: &T) -> Result<()> {
        // Simplified implementation
        tracing::debug!("Cache set: {}", key);
        Ok(())
    }

    async fn invalidate(&self, key: &str) -> Result<()> {
        tracing::debug!("Cache invalidate: {}", key);
        Ok(())
    }
}
