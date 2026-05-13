//! Configuration loader

use pristine_core::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration loader
pub struct ConfigLoader;

impl ConfigLoader {
    /// Create a new config loader
    pub fn new() -> Self {
        Self
    }
    
    /// Load configuration from a file
    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<PristineConfig> {
        tracing::info!("Loading configuration from: {:?}", path.as_ref());
        
        // Placeholder implementation
        Ok(PristineConfig::default())
    }
    
    /// Load configuration from environment variables
    pub fn load_from_env(&self) -> Result<PristineConfig> {
        tracing::debug!("Loading configuration from environment");
        
        // Placeholder implementation
        Ok(PristineConfig::default())
    }
    
    /// Load configuration with default search paths
    pub fn load(&self) -> Result<PristineConfig> {
        // Search order:
        // 1. PRISTINE_CONFIG env var
        // 2. .pristine.yaml in current directory
        // 3. .pristine.yml in current directory
        // 4. .pristine.json in current directory
        // 5. ~/.config/pristine/config.yaml
        
        if let Ok(config_path) = std::env::var("PRISTINE_CONFIG") {
            return self.load_from_file(config_path);
        }
        
        let search_paths = [
            ".pristine.yaml",
            ".pristine.yml",
            ".pristine.json",
        ];
        
        for path in &search_paths {
            if std::path::Path::new(path).exists() {
                return self.load_from_file(path);
            }
        }
        
        // Fall back to environment variables
        self.load_from_env()
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Pristine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PristineConfig {
    /// Default profile
    pub default_profile: String,
    /// Output configuration
    pub output: OutputConfig,
    /// Budget configuration
    pub budget: BudgetConfig,
    /// Policy configuration
    pub policy: PolicyConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Server configuration
    pub server: ServerConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Default for PristineConfig {
    fn default() -> Self {
        Self {
            default_profile: "overview".to_string(),
            output: OutputConfig::default(),
            budget: BudgetConfig::default(),
            policy: PolicyConfig::default(),
            cache: CacheConfig::default(),
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub directory: String,
    pub max_size: u64,
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

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_origins: vec!["http://localhost:3000".to_string()],
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
        }
    }
}
