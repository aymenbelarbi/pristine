//! GitHub repository acquirer

use async_trait::async_trait;
use pristine_core::*;
use std::path::PathBuf;

/// GitHub repository acquirer
pub struct GitHubAcquirer {
    clone_dir: PathBuf,
    token: Option<String>,
}

impl GitHubAcquirer {
    /// Create a new GitHub acquirer
    pub fn new(clone_dir: PathBuf, token: Option<String>) -> Self {
        Self { clone_dir, token }
    }
    
    /// Parse a GitHub URL into owner and repo
    fn parse_url(&self, url: &str) -> Result<(String, String)> {
        // Handle various GitHub URL formats:
        // - https://github.com/owner/repo
        // - https://github.com/owner/repo.git
        // - git@github.com:owner/repo.git
        
        let url = url.trim_end_matches(".git");
        
        if url.starts_with("git@github.com:") {
            let parts: Vec<&str> = url.trim_start_matches("git@github.com:").split('/').collect();
            if parts.len() == 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }
        
        if url.starts_with("https://github.com/") {
            let parts: Vec<&str> = url.trim_start_matches("https://github.com/").split('/').collect();
            if parts.len() >= 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }
        
        Err(PristineError::InvalidSource(format!("Invalid GitHub URL: {}", url)))
    }
}

#[async_trait]
impl Acquire for GitHubAcquirer {
    async fn acquire(&self, source: &SourceRef) -> Result<RepoSnapshot> {
        tracing::info!("Acquiring GitHub repository: {}", source.locator);
        
        let (owner, repo) = self.parse_url(&source.locator)?;
        tracing::debug!("Parsed GitHub URL: {}/{}", owner, repo);
        
        // Placeholder implementation
        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let root = self.clone_dir.join(&snapshot_id);
        
        Ok(RepoSnapshot {
            snapshot_id,
            root,
            revision: source.revision.clone(),
            source: source.clone(),
            created_at: chrono::Utc::now(),
        })
    }

    async fn check_access(&self, source: &SourceRef) -> Result<bool> {
        match self.parse_url(&source.locator) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
