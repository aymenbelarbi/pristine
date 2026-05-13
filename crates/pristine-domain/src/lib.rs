//! Pristine Domain - Business logic
//!
//! This crate contains the core business logic for Pristine,
//! including the main engine that orchestrates the pipeline.

pub mod engine;
pub mod pipeline;
pub mod config;

pub use engine::PristineEngine;
pub use pipeline::Pipeline;
pub use config::EngineConfig;
