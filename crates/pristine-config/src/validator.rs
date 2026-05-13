//! Configuration validator

use crate::loader::PristineConfig;

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    /// Create a new config validator
    pub fn new() -> Self {
        Self
    }
    
    /// Validate configuration
    pub fn validate(&self, config: &PristineConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validate profile
        let valid_profiles = ["overview", "pack", "review_diff", "agent", "safe_share"];
        if !valid_profiles.contains(&config.default_profile.as_str()) {
            errors.push(format!(
                "Invalid default_profile: {}. Must be one of: {:?}",
                config.default_profile, valid_profiles
            ));
        }
        
        // Validate budget
        if let Some(max_tokens) = config.budget.max_tokens {
            if max_tokens < 1000 {
                errors.push("max_tokens must be at least 1000".to_string());
            }
        }
        
        // Validate server port
        if config.server.port == 0 {
            errors.push("server port cannot be 0".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}
