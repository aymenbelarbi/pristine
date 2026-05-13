//! Ignore rules handling

use std::path::Path;

/// Ignore rules for file filtering
pub struct IgnoreRules {
    patterns: Vec<String>,
    use_gitignore: bool,
    use_pristineignore: bool,
}

impl IgnoreRules {
    /// Create new ignore rules
    pub fn new() -> Self {
        Self {
            patterns: vec![],
            use_gitignore: true,
            use_pristineignore: true,
        }
    }
    
    /// Add a pattern
    pub fn add_pattern(&mut self, pattern: impl Into<String>) {
        self.patterns.push(pattern.into());
    }
    
    /// Check if a path should be ignored
    pub fn is_ignored(&self, path: &Path) -> bool {
        // Placeholder implementation
        let path_str = path.to_string_lossy();
        
        // Check built-in patterns
        if path_str.contains("node_modules")
            || path_str.contains("target")
            || path_str.contains(".git")
            || path_str.contains("__pycache__")
        {
            return true;
        }
        
        // Check custom patterns
        for pattern in &self.patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
}

impl Default for IgnoreRules {
    fn default() -> Self {
        Self::new()
    }
}
