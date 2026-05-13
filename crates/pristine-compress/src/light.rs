//! Light compression

/// Apply light compression to content
pub fn compress(content: &str) -> String {
    let mut result = String::new();
    let mut prev_blank = false;
    let mut in_comment_block = false;
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Handle comment blocks
        if trimmed.starts_with("/*") || trimmed.starts_with("/**") {
            in_comment_block = true;
        }
        if trimmed.ends_with("*/") {
            in_comment_block = false;
            continue;
        }
        if in_comment_block {
            continue;
        }
        
        // Skip single-line comments (but keep doc comments)
        if (trimmed.starts_with("//") && !trimmed.starts_with("///") && !trimmed.starts_with("//!"))
            || trimmed.starts_with('#') && !trimmed.starts_with("#[")
            || trimmed.starts_with("--")
        {
            continue;
        }
        
        // Handle blank lines
        if trimmed.is_empty() {
            if !prev_blank {
                result.push('\n');
                prev_blank = true;
            }
            continue;
        }
        
        prev_blank = false;
        result.push_str(line);
        result.push('\n');
    }
    
    result
}
