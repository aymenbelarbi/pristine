//! Selection scoring

use async_trait::async_trait;
use pristine_core::*;

/// Scoring context for file selection
#[derive(Debug, Default)]
pub struct ScoringContext<'a> {
    pub query: Option<&'a str>,
    pub changed_files: &'a [String],
}

/// Selection scorer
pub struct SelectionScorer {
    profile: Profile,
}

impl SelectionScorer {
    /// Create a new selection scorer
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
    
    /// Score a single file
    pub fn score(&self, file: &FileRecord, context: &ScoringContext) -> f64 {
        let mut score: f64 = 0.0;
        
        // Entrypoint bonus
        if file.tags.contains(&FileTag::Entrypoint) {
            score += 25.0;
        }
        
        // Manifest bonus
        if file.tags.contains(&FileTag::Manifest) {
            score += 20.0;
        }
        
        // Source file bonus
        if file.tags.contains(&FileTag::Source) {
            score += 10.0;
        }
        
        // Test file bonus
        if file.tags.contains(&FileTag::Test) {
            score += 5.0;
        }
        
        // Config file bonus
        if file.tags.contains(&FileTag::Config) {
            score += 5.0;
        }
        
        // Query match bonus
        if let Some(query) = context.query {
            if file.path.to_lowercase().contains(&query.to_lowercase()) {
                score += 15.0;
            }
        }
        
        // Changed file bonus
        if context.changed_files.contains(&file.path) {
            score += 40.0;
        }
        
        // Penalties
        if file.tags.contains(&FileTag::Generated) {
            score -= 25.0;
        }
        
        if file.tags.contains(&FileTag::Vendor) {
            score -= 35.0;
        }
        
        if file.is_binary {
            score -= 40.0;
        }
        
        // Depth penalty (prefer shallower files)
        score -= file.depth as f64 * 2.0;
        
        score
    }
    
    /// Score an entire catalog
    pub fn score_catalog(
        &self,
        catalog: &ClassifiedCatalog,
        context: &ScoringContext,
    ) -> Vec<SelectionDecision> {
        catalog
            .classified_files
            .iter()
            .map(|file| {
                let score = self.score(&file.record, context);
                let inclusion = self.score_to_inclusion(score);
                
                SelectionDecision {
                    path: file.record.path.clone(),
                    inclusion,
                    reasons: self.get_reasons(&file.record, context),
                    estimated_tokens: None,
                    score,
                }
            })
            .collect()
    }
    
    fn score_to_inclusion(&self, score: f64) -> InclusionLevel {
        match score {
            s if s >= 30.0 => InclusionLevel::Full,
            s if s >= 15.0 => InclusionLevel::Compressed,
            s if s >= 5.0 => InclusionLevel::Summary,
            s if s >= 0.0 => InclusionLevel::TreeOnly,
            _ => InclusionLevel::Excluded,
        }
    }
    
    fn get_reasons(&self, file: &FileRecord, context: &ScoringContext) -> Vec<SelectionReason> {
        let mut reasons = Vec::new();
        
        if file.tags.contains(&FileTag::Entrypoint) {
            reasons.push(SelectionReason::Entrypoint);
        }
        
        if let Some(query) = context.query {
            if file.path.to_lowercase().contains(&query.to_lowercase()) {
                reasons.push(SelectionReason::QueryMatch);
            }
        }
        
        if context.changed_files.contains(&file.path) {
            reasons.push(SelectionReason::ChangedFile);
        }
        
        if file.tags.contains(&FileTag::Manifest) {
            reasons.push(SelectionReason::FrameworkManifest);
        }
        
        if reasons.is_empty() {
            reasons.push(SelectionReason::HighScore);
        }
        
        reasons
    }
}

#[async_trait]
impl Select for SelectionScorer {
    async fn select(
        &self,
        catalog: &ClassifiedCatalog,
        request: &ArtifactRequest,
    ) -> Result<SelectionPlan> {
        tracing::info!("Selecting files for profile: {:?}", request.profile);
        
        let context = ScoringContext {
            query: request.query.as_deref(),
            changed_files: &[],
        };
        
        let decisions = self.score_catalog(catalog, &context);
        let total_files = decisions.len() as u32;
        let total_tokens: u32 = decisions
            .iter()
            .filter_map(|d| d.estimated_tokens)
            .sum();
        
        Ok(SelectionPlan {
            decisions,
            total_tokens,
            total_files,
        })
    }
}
