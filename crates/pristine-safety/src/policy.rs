//! Policy enforcement

use async_trait::async_trait;
use pristine_core::*;

/// Policy enforcer
pub struct PolicyEnforcer;

impl PolicyEnforcer {
    /// Create a new policy enforcer
    pub fn new() -> Self {
        Self
    }
}

impl Default for PolicyEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PolicyEnforce for PolicyEnforcer {
    async fn check_file(&self, file: &FileRecord, policy: &PolicyConfig) -> Result<PolicyResult> {
        // Check block patterns using simple glob matching
        for pattern in &policy.block_patterns {
            if glob_match(pattern, &file.path) {
                return Ok(PolicyResult {
                    allowed: false,
                    action: PolicyAction::Block,
                    reason: Some(format!("Matches block pattern: {}", pattern)),
                });
            }
        }
        
        // Check file size
        if let Some(max_size) = policy.max_file_size {
            if file.size_bytes > max_size {
                return Ok(PolicyResult {
                    allowed: false,
                    action: PolicyAction::Block,
                    reason: Some(format!(
                        "File size {} exceeds maximum {}",
                        file.size_bytes, max_size
                    )),
                });
            }
        }
        
        // Check secrets risk tag
        if policy.scan_secrets && file.tags.contains(&FileTag::SecretsRisk) {
            match policy.mode {
                PolicyMode::Allow => Ok(PolicyResult {
                    allowed: true,
                    action: PolicyAction::Allow,
                    reason: None,
                }),
                PolicyMode::Redact => Ok(PolicyResult {
                    allowed: true,
                    action: PolicyAction::Redact,
                    reason: Some("File flagged as secrets risk".to_string()),
                }),
                PolicyMode::Fail => Ok(PolicyResult {
                    allowed: false,
                    action: PolicyAction::Block,
                    reason: Some("File flagged as secrets risk".to_string()),
                }),
            }
        } else {
            Ok(PolicyResult {
                allowed: true,
                action: PolicyAction::Allow,
                reason: None,
            })
        }
    }
}

/// Simple glob pattern matching
fn glob_match(pattern: &str, path: &str) -> bool {
    // Handle ** (match any directories)
    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            return path.starts_with(prefix) && path.ends_with(suffix);
        }
    }
    
    // Handle * (match any characters except /)
    if pattern.contains('*') {
        let regex_pattern = pattern
            .replace(".", "\\.")
            .replace("*", "[^/]*");
        if let Ok(regex) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
            return regex.is_match(path);
        }
    }
    
    // Exact match
    path == pattern
}
