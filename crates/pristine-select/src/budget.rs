//! Budget fitting

use pristine_core::*;

/// Budget fitter for adjusting selection to fit budget constraints
pub struct BudgetFitter {
    config: BudgetConfig,
}

impl BudgetFitter {
    /// Create a new budget fitter
    pub fn new(config: BudgetConfig) -> Self {
        Self { config }
    }
    
    /// Fit selection to budget
    pub fn fit(&self, plan: &mut SelectionPlan) {
        // Apply max files constraint
        if let Some(max_files) = self.config.max_files {
            if plan.decisions.len() > max_files as usize {
                // Sort by score and keep top N
                plan.decisions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                plan.decisions.truncate(max_files as usize);
            }
        }
        
        // Apply token budget
        if let Some(max_tokens) = self.config.max_tokens {
            self.fit_tokens(plan, max_tokens);
        }
    }
    
    fn fit_tokens(&self, plan: &mut SelectionPlan, max_tokens: u32) {
        let mut total_tokens = plan.total_tokens;
        
        if total_tokens <= max_tokens {
            return;
        }
        
        // First, downgrade compressed files to summary
        for decision in plan.decisions.iter_mut() {
            if total_tokens <= max_tokens {
                break;
            }
            
            if decision.inclusion == InclusionLevel::Compressed {
                decision.inclusion = InclusionLevel::Summary;
                decision.reasons.push(SelectionReason::BudgetDowngraded);
                // Estimate token reduction
                if let Some(tokens) = decision.estimated_tokens {
                    total_tokens = total_tokens.saturating_sub(tokens / 2);
                }
            }
        }
        
        // Then, downgrade summary files to tree-only
        for decision in plan.decisions.iter_mut() {
            if total_tokens <= max_tokens {
                break;
            }
            
            if decision.inclusion == InclusionLevel::Summary {
                decision.inclusion = InclusionLevel::TreeOnly;
                // Estimate token reduction
                if let Some(tokens) = decision.estimated_tokens {
                    total_tokens = total_tokens.saturating_sub(tokens);
                }
            }
        }
        
        // Finally, exclude low-scoring files if still over budget
        plan.decisions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        while total_tokens > max_tokens && !plan.decisions.is_empty() {
            if let Some(decision) = plan.decisions.last_mut() {
                if decision.inclusion == InclusionLevel::TreeOnly {
                    if let Some(tokens) = decision.estimated_tokens {
                        total_tokens = total_tokens.saturating_sub(tokens);
                    }
                    decision.inclusion = InclusionLevel::Excluded;
                    decision.reasons.push(SelectionReason::BudgetDowngraded);
                }
            }
            plan.decisions.pop();
        }
        
        plan.total_tokens = total_tokens;
    }
}
