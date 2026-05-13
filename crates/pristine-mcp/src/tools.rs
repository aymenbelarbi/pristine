//! MCP tool definitions

use serde::{Deserialize, Serialize};

/// Tool input for overview_repo
#[derive(Debug, Serialize, Deserialize)]
pub struct OverviewRepoInput {
    pub source: String,
    pub revision: Option<String>,
    pub subpath: Option<String>,
}

/// Tool input for pack_context
#[derive(Debug, Serialize, Deserialize)]
pub struct PackContextInput {
    pub source: String,
    pub query: String,
    pub max_tokens: Option<u32>,
    pub include_patterns: Option<Vec<String>>,
    pub exclude_patterns: Option<Vec<String>>,
}

/// Tool input for review_diff
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewDiffInput {
    pub source: String,
    pub base: String,
    pub head: String,
    pub explain: Option<bool>,
}

/// Tool input for search_files
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFilesInput {
    pub source: String,
    pub query: String,
    pub file_pattern: Option<String>,
    pub max_results: Option<u32>,
}

/// Tool input for read_file
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadFileInput {
    pub source: String,
    pub file_path: String,
    pub revision: Option<String>,
    pub compression: Option<String>,
}

/// Tool input for safe_share
#[derive(Debug, Serialize, Deserialize)]
pub struct SafeShareInput {
    pub source: String,
    pub policy: Option<String>,
    pub max_tokens: Option<u32>,
}
