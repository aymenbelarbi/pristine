//! Engine configuration

use serde::{Deserialize, Serialize};

/// Engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Maximum concurrent operations
    pub max_concurrent: usize,
    /// Default profile
    pub default_profile: String,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent: num_cpus::get(),
            default_profile: "overview".to_string(),
            cache: CacheConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache directory
    pub directory: String,
    /// Maximum cache size in bytes
    pub max_size: u64,
    /// Cache TTL in seconds
    pub ttl: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: ".pristine/cache".to_string(),
            max_size: 1_073_741_824, // 1GB
            ttl: 86_400,             // 24 hours
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: LogFormat,
    /// Log output
    pub output: LogOutput,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: LogFormat::Pretty,
            output: LogOutput::Stdout,
        }
    }
}

/// Log format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
}

/// Log output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}
