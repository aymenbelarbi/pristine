//! Core traits for Pristine

use async_trait::async_trait;
use crate::types::*;
use crate::error::Result;

/// Trait for acquiring repositories
#[async_trait]
pub trait Acquire: Send + Sync {
    /// Acquire a repository from a source
    async fn acquire(&self, source: &SourceRef) -> Result<RepoSnapshot>;
    
    /// Check if a source is accessible
    async fn check_access(&self, source: &SourceRef) -> Result<bool>;
}

/// Trait for inventorying repositories
#[async_trait]
pub trait Inventory: Send + Sync {
    /// Inventory a repository snapshot
    async fn inventory(&self, snapshot: &RepoSnapshot) -> Result<FileCatalog>;
}

/// Trait for classifying files
#[async_trait]
pub trait Classify: Send + Sync {
    /// Classify files in a catalog
    async fn classify(&self, catalog: &FileCatalog) -> Result<ClassifiedCatalog>;
}

/// Trait for selecting files
#[async_trait]
pub trait Select: Send + Sync {
    /// Select files based on request
    async fn select(
        &self,
        catalog: &ClassifiedCatalog,
        request: &ArtifactRequest,
    ) -> Result<SelectionPlan>;
}

/// Trait for compressing file content
#[async_trait]
pub trait Compress: Send + Sync {
    /// Compress a file
    async fn compress(
        &self,
        content: &str,
        language: Option<Language>,
        mode: CompressionMode,
    ) -> Result<String>;
}

/// Trait for assembling artifacts
#[async_trait]
pub trait Assemble: Send + Sync {
    /// Assemble an artifact from selection plan
    async fn assemble(
        &self,
        plan: &SelectionPlan,
        request: &ArtifactRequest,
    ) -> Result<ContextArtifact>;
}

/// Trait for rendering artifacts
#[async_trait]
pub trait Render: Send + Sync {
    /// Render an artifact to string
    async fn render(&self, artifact: &ContextArtifact) -> Result<String>;
}

/// Trait for caching
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get a cached value
    async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;
    
    /// Set a cached value
    async fn set<T: serde::Serialize + Send + Sync>(&self, key: &str, value: &T) -> Result<()>;
    
    /// Invalidate a cached value
    async fn invalidate(&self, key: &str) -> Result<()>;
}

/// Trait for secret scanning
#[async_trait]
pub trait SecretScan: Send + Sync {
    /// Scan content for secrets
    async fn scan(&self, content: &str, file_path: &str) -> Result<Vec<SecretFinding>>;
}

/// Secret finding
#[derive(Debug, Clone)]
pub struct SecretFinding {
    pub pattern_name: String,
    pub severity: SecretSeverity,
    pub line_number: u32,
    pub column: u32,
    pub matched_text: String,
}

/// Secret severity
#[derive(Debug, Clone, PartialEq)]
pub enum SecretSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Trait for policy enforcement
#[async_trait]
pub trait PolicyEnforce: Send + Sync {
    /// Check if a file is allowed by policy
    async fn check_file(&self, file: &FileRecord, policy: &PolicyConfig) -> Result<PolicyResult>;
}

/// Policy check result
#[derive(Debug, Clone)]
pub struct PolicyResult {
    pub allowed: bool,
    pub action: PolicyAction,
    pub reason: Option<String>,
}

/// Policy action
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyAction {
    Allow,
    Redact,
    Block,
}
