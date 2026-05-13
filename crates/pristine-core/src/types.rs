//! Core domain types for Pristine

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Source reference for repository location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceRef {
    pub kind: SourceKind,
    pub locator: String,
    pub revision: Option<String>,
    pub subpath: Option<String>,
}

/// Type of source repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind {
    Local,
    GitHub,
    GitLab,
}

/// Request for artifact generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRequest {
    pub source: SourceRef,
    pub profile: Profile,
    pub query: Option<String>,
    pub diff: Option<DiffSpec>,
    pub policy: PolicyConfig,
    pub budget: BudgetConfig,
    pub output: OutputConfig,
}

/// Repository snapshot after acquisition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSnapshot {
    pub snapshot_id: String,
    pub root: PathBuf,
    pub revision: Option<String>,
    pub source: SourceRef,
    pub created_at: DateTime<Utc>,
}

/// File record from inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub path: String,
    pub relative_path: String,
    pub size_bytes: u64,
    pub language: Option<Language>,
    pub is_binary: bool,
    pub tags: Vec<FileTag>,
    pub depth: u32,
    pub last_modified: Option<DateTime<Utc>>,
}

/// Selection decision for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionDecision {
    pub path: String,
    pub inclusion: InclusionLevel,
    pub reasons: Vec<SelectionReason>,
    pub estimated_tokens: Option<u32>,
    pub score: f64,
}

/// Final context artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextArtifact {
    pub artifact_type: ArtifactType,
    pub metadata: ArtifactMetadata,
    pub file_units: Vec<RenderedFileUnit>,
    pub stats: ArtifactStats,
    pub warnings: Vec<ArtifactWarning>,
}

/// Artifact metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMetadata {
    pub artifact_version: String,
    pub generator_version: String,
    pub source: SourceRef,
    pub revision: String,
    pub profile: Profile,
    pub policy_mode: PolicyMode,
    pub generated_at: DateTime<Utc>,
    pub fingerprint: String,
}

/// Rendered file unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedFileUnit {
    pub path: String,
    pub content: String,
    pub inclusion: InclusionLevel,
    pub language: Option<Language>,
    pub line_count: u32,
    pub token_count: u32,
    pub reasons: Vec<SelectionReason>,
    pub compression_mode: CompressionMode,
}

/// Artifact statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactStats {
    pub total_files: u32,
    pub included_files: u32,
    pub full_files: u32,
    pub compressed_files: u32,
    pub summary_files: u32,
    pub tree_only_files: u32,
    pub excluded_files: u32,
    pub total_bytes: u64,
    pub total_tokens: u32,
    pub compression_ratio: f64,
}

/// Artifact warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactWarning {
    pub level: WarningLevel,
    pub message: String,
    pub file: Option<String>,
}

/// Diff specification for review artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSpec {
    pub base: String,
    pub head: String,
}

/// Profile for artifact generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Profile {
    Overview,
    Pack,
    ReviewDiff,
    Agent,
    SafeShare,
}

/// Inclusion level for a file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InclusionLevel {
    Full,
    Compressed,
    Summary,
    TreeOnly,
    Excluded,
}

/// Compression mode for file content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompressionMode {
    None,
    Light,
    Structural,
    Summary,
}

/// Policy mode for safety
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyMode {
    Allow,
    Redact,
    Fail,
}

/// File tags for classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileTag {
    Source,
    Config,
    Lockfile,
    Test,
    Docs,
    Generated,
    Vendor,
    Notebook,
    Binary,
    Fixture,
    Migration,
    SecretsRisk,
    Entrypoint,
    Manifest,
}

/// Reason for file selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SelectionReason {
    Entrypoint,
    QueryMatch,
    ChangedFile,
    AdjacentTest,
    FrameworkManifest,
    PolicyExcluded,
    BudgetDowngraded,
    HighScore,
    UserIncluded,
    UserExcluded,
}

/// Type of artifact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    Overview,
    Task,
    Review,
    Agent,
    Safe,
}

/// Programming language
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Go,
    Java,
    C,
    Cpp,
    Csharp,
    Ruby,
    Php,
    Swift,
    Kotlin,
    Scala,
    Elixir,
    Haskell,
    Other(String),
}

/// Warning level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WarningLevel {
    Info,
    Warning,
    Error,
}

/// Policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub mode: PolicyMode,
    pub allow_patterns: Vec<String>,
    pub block_patterns: Vec<String>,
    pub redact_patterns: Vec<String>,
    pub max_file_size: Option<u64>,
    pub scan_secrets: bool,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            mode: PolicyMode::Allow,
            allow_patterns: vec![],
            block_patterns: vec![],
            redact_patterns: vec![],
            max_file_size: None,
            scan_secrets: true,
        }
    }
}

/// Budget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub max_tokens: Option<u32>,
    pub max_bytes: Option<u64>,
    pub max_files: Option<u32>,
    pub compression_preference: CompressionMode,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            max_tokens: Some(50_000),
            max_bytes: Some(10_485_760), // 10MB
            max_files: Some(100),
            compression_preference: CompressionMode::Light,
        }
    }
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub include_tree: bool,
    pub include_stats: bool,
    pub include_reasons: bool,
    pub theme: Option<String>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Markdown,
            include_tree: true,
            include_stats: true,
            include_reasons: true,
            theme: None,
        }
    }
}

/// Output format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Markdown,
    Xml,
    Text,
}

/// File catalog from inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCatalog {
    pub snapshot_id: String,
    pub files: Vec<FileRecord>,
    pub total_count: u32,
    pub total_bytes: u64,
}

/// Classified catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedCatalog {
    pub catalog: FileCatalog,
    pub classified_files: Vec<ClassifiedFile>,
}

/// Classified file with tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedFile {
    pub record: FileRecord,
    pub tags: Vec<FileTag>,
    pub framework: Option<String>,
    pub is_entrypoint: bool,
}

/// Selection plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionPlan {
    pub decisions: Vec<SelectionDecision>,
    pub total_tokens: u32,
    pub total_files: u32,
}

/// Compressed catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedCatalog {
    pub plan: SelectionPlan,
    pub compressed_files: Vec<CompressedFile>,
}

/// Compressed file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedFile {
    pub path: String,
    pub original_content: String,
    pub compressed_content: String,
    pub mode: CompressionMode,
    pub original_tokens: u32,
    pub compressed_tokens: u32,
}

/// Rendered artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedArtifact {
    pub artifact: ContextArtifact,
    pub content: String,
    pub format: OutputFormat,
}

impl SourceRef {
    /// Create a new local source reference
    pub fn local(path: impl Into<String>) -> Self {
        Self {
            kind: SourceKind::Local,
            locator: path.into(),
            revision: None,
            subpath: None,
        }
    }

    /// Create a new GitHub source reference
    pub fn github(url: impl Into<String>) -> Self {
        Self {
            kind: SourceKind::GitHub,
            locator: url.into(),
            revision: None,
            subpath: None,
        }
    }

    /// Set the revision
    pub fn with_revision(mut self, revision: impl Into<String>) -> Self {
        self.revision = Some(revision.into());
        self
    }

    /// Set the subpath
    pub fn with_subpath(mut self, subpath: impl Into<String>) -> Self {
        self.subpath = Some(subpath.into());
        self
    }
}

impl RepoSnapshot {
    /// Create a new repository snapshot
    pub fn new(root: PathBuf, source: SourceRef) -> Self {
        Self {
            snapshot_id: Uuid::new_v4().to_string(),
            root,
            revision: source.revision.clone(),
            source,
            created_at: Utc::now(),
        }
    }
}

impl FileRecord {
    /// Create a new file record
    pub fn new(path: impl Into<String>, size_bytes: u64) -> Self {
        let path = path.into();
        let relative_path = path.clone();
        Self {
            path,
            relative_path,
            size_bytes,
            language: None,
            is_binary: false,
            tags: vec![],
            depth: 0,
            last_modified: None,
        }
    }

    /// Check if the file is a source file
    pub fn is_source(&self) -> bool {
        self.tags.contains(&FileTag::Source)
    }

    /// Check if the file is a test file
    pub fn is_test(&self) -> Self {
        self.tags.contains(&FileTag::Test)
    }

    /// Check if the file is a config file
    pub fn is_config(&self) -> bool {
        self.tags.contains(&FileTag::Config)
    }
}

impl ArtifactStats {
    /// Create empty stats
    pub fn empty() -> Self {
        Self {
            total_files: 0,
            included_files: 0,
            full_files: 0,
            compressed_files: 0,
            summary_files: 0,
            tree_only_files: 0,
            excluded_files: 0,
            total_bytes: 0,
            total_tokens: 0,
            compression_ratio: 1.0,
        }
    }
}
