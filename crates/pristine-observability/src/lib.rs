//! Pristine Observability - Metrics, logging, and tracing
//!
//! This crate provides observability infrastructure for Pristine,
//! including metrics collection, structured logging, and distributed tracing.

pub mod metrics;
pub mod logging;
pub mod tracing;

pub use metrics::MetricsRegistry;
pub use logging::init_logging;
pub use tracing::init_tracing;
