//! Secret scanner

use async_trait::async_trait;
use pristine_core::*;
use regex::Regex;

/// Secret scanner for detecting sensitive information
pub struct SecretScanner {
    patterns: Vec<SecretPattern>,
}

struct SecretPattern {
    name: String,
    pattern: Regex,
    severity: SecretSeverity,
    description: String,
}

impl SecretScanner {
    /// Create a new secret scanner with default patterns
    pub fn new() -> Self {
        let patterns = vec![
            SecretPattern {
                name: "aws_access_key".to_string(),
                pattern: Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
                severity: SecretSeverity::Critical,
                description: "AWS Access Key ID".to_string(),
            },
            SecretPattern {
                name: "github_token".to_string(),
                pattern: Regex::new(r"gh[pousr]_[A-Za-z0-9_]{36,}").unwrap(),
                severity: SecretSeverity::Critical,
                description: "GitHub Personal Access Token".to_string(),
            },
            SecretPattern {
                name: "private_key".to_string(),
                pattern: Regex::new(r"-----BEGIN (RSA |EC |DSA |OPENSSH )?PRIVATE KEY-----").unwrap(),
                severity: SecretSeverity::Critical,
                description: "Private Key".to_string(),
            },
            SecretPattern {
                name: "api_key_generic".to_string(),
                pattern: Regex::new(r"(?i)(api[_-]?key|apikey)\s*[:=]\s*['""][a-zA-Z0-9_\-]{32,}['""]").unwrap(),
                severity: SecretSeverity::High,
                description: "Generic API Key".to_string(),
            },
            SecretPattern {
                name: "password".to_string(),
                pattern: Regex::new(r"(?i)(password|passwd|pwd)\s*[:=]\s*['""][^'""]+['""]").unwrap(),
                severity: SecretSeverity::High,
                description: "Password".to_string(),
            },
        ];
        
        Self { patterns }
    }
    
    /// Add a custom pattern
    pub fn add_pattern(&mut self, name: &str, pattern: &str, severity: SecretSeverity, description: &str) -> Result<()> {
        let regex = Regex::new(pattern)
            .map_err(|e| PristineError::General(format!("Invalid regex pattern: {}", e)))?;
        
        self.patterns.push(SecretPattern {
            name: name.to_string(),
            pattern: regex,
            severity,
            description: description.to_string(),
        });
        
        Ok(())
    }
}

impl Default for SecretScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SecretScan for SecretScanner {
    async fn scan(&self, content: &str, file_path: &str) -> Result<Vec<SecretFinding>> {
        let mut findings = Vec::new();
        
        for (line_num, line in content.lines().enumerate() {
            for pattern in &self.patterns {
                for mat in pattern.pattern.find_iter(line) {
                    findings.push(SecretFinding {
                        pattern_name: pattern.name.clone(),
                        severity: pattern.severity.clone(),
                        line_number: (line_num + 1) as u32,
                        column: (mat.start() + 1) as u32,
                        matched_text: mat.as_str().to_string(),
                    });
                }
            }
        }
        
        if !findings.is_empty() {
            tracing::warn!(
                file = file_path,
                count = findings.len(),
                "Secrets detected in file"
            );
        }
        
        Ok(findings)
    }
}
