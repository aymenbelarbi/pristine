//! Metrics registry

use metrics::{counter, gauge, histogram};

/// Metrics registry for Pristine
pub struct MetricsRegistry;

impl MetricsRegistry {
    /// Create a new metrics registry
    pub fn new() -> Self {
        Self
    }
    
    /// Record a request
    pub fn record_request(&self, profile: &str) {
        counter!("pristine_requests_total", "profile" => profile.to_string()).increment(1);
    }
    
    /// Record request duration
    pub fn record_request_duration(&self, profile: &str, duration_ms: f64) {
        histogram!("pristine_request_duration_ms", "profile" => profile.to_string()).record(duration_ms);
    }
    
    /// Record stage duration
    pub fn record_stage_duration(&self, stage: &str, duration_ms: f64) {
        histogram!("pristine_stage_duration_ms", "stage" => stage.to_string()).record(duration_ms);
    }
    
    /// Record cache hit
    pub fn record_cache_hit(&self, cache_type: &str) {
        counter!("pristine_cache_hits_total", "cache_type" => cache_type.to_string()).increment(1);
    }
    
    /// Record cache miss
    pub fn record_cache_miss(&self, cache_type: &str) {
        counter!("pristine_cache_misses_total", "cache_type" => cache_type.to_string()).increment(1);
    }
    
    /// Record files processed
    pub fn record_files_processed(&self, count: u64) {
        counter!("pristine_files_processed_total").increment(count);
    }
    
    /// Record tokens emitted
    pub fn record_tokens_emitted(&self, count: u64) {
        counter!("pristine_tokens_emitted_total").increment(count);
    }
    
    /// Record secret detected
    pub fn record_secret_detected(&self, pattern: &str) {
        counter!("pristine_secrets_detected_total", "pattern" => pattern.to_string()).increment(1);
    }
    
    /// Set active requests gauge
    pub fn set_active_requests(&self, count: u64) {
        gauge!("pristine_active_requests").set(count as f64);
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}
