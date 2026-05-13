//! Main compressor

use async_trait::async_trait;
use pristine_core::*;

/// Compressor for file content
pub struct Compressor {
    default_mode: CompressionMode,
}

impl Compressor {
    /// Create a new compressor
    pub fn new(default_mode: CompressionMode) -> Self {
        Self { default_mode }
    }
    
    /// Compress content
    pub async fn compress(
        &self,
        content: &str,
        language: Option<Language>,
        mode: CompressionMode,
    ) -> Result<String> {
        match mode {
            CompressionMode::None => Ok(content.to_string()),
            CompressionMode::Light => self.compress_light(content),
            CompressionMode::Structural => self.compress_structural(content, language),
            CompressionMode::Summary => self.compress_summary(content, language),
        }
    }
    
    fn compress_light(&self, content: &str) -> Result<String> {
        // Remove excessive blank lines and normalize whitespace
        let mut result = String::new();
        let mut prev_blank = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !prev_blank {
                    result.push('\n');
                    prev_blank = true;
                }
            } else {
                result.push_str(line);
                result.push('\n');
                prev_blank = false;
            }
        }
        
        Ok(result)
    }
    
    fn compress_structural(&self, content: &str, language: Option<Language>) -> Result<String> {
        // Placeholder: preserve structure, collapse bodies
        // In a full implementation, this would use tree-sitter
        self.compress_light(content)
    }
    
    fn compress_summary(&self, content: &str, _language: Option<Language>) -> Result<String> {
        // Placeholder: extract symbols and metadata
        let line_count = content.lines().count();
        Ok(format!("[Summary: {} lines]", line_count))
    }
}

impl Default for Compressor {
    fn default() -> Self {
        Self::new(CompressionMode::Light)
    }
}

#[async_trait]
impl Compress for Compressor {
    async fn compress(
        &self,
        content: &str,
        language: Option<Language>,
        mode: CompressionMode,
    ) -> Result<String> {
        self.compress(content, language, mode).await
    }
}
