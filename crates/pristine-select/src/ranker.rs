//! File ranker

use pristine_core::*;

/// File ranker for ordering files by relevance
pub struct FileRanker;

impl FileRanker {
    /// Create a new file ranker
    pub fn new() -> Self {
        Self
    }
    
    /// Rank files by score
    pub fn rank(&self, decisions: &mut [SelectionDecision]) {
        decisions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }
    
    /// Get top N files
    pub fn top_n(&self, decisions: &[SelectionDecision], n: usize) -> Vec<&SelectionDecision> {
        let mut sorted: Vec<_> = decisions.iter().collect();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        sorted.into_iter().take(n).collect()
    }
}

impl Default for FileRanker {
    fn default() -> Self {
        Self::new()
    }
}
