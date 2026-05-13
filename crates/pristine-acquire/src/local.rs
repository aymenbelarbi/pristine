//! Local directory acquirer

use async_trait::async_trait;
use pristine_core::*;
use std::path::PathBuf;

/// Local directory acquirer
pub struct LocalAcquirer;

impl LocalAcquirer {
    /// Create a new local acquirer
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocalAcquirer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Acquire for LocalAcquirer {
    async fn acquire(&self, source: &SourceRef) -> Result<RepoSnapshot> {
        tracing::info!("Acquiring local directory: {}", source.locator);
        
        let path = PathBuf::from(&source.locator);
        if !path.exists() {
            return Err(PristineError::SourceNotFound(source.locator.clone()));
        }
        
        if !path.is_dir() {
            return Err(PristineError::InvalidSource(
                format!("{} is not a directory", source.locator)
            ));
        }
        
        Ok(RepoSnapshot::new(path, source.clone()))
    }

    async fn check_access(&self, source: &SourceRef) -> Result<bool> {
        let path = PathBuf::from(&source.locator);
        Ok(path.exists() && path.is_dir())
    }
}
