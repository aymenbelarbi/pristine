//! Pristine Select - File selection engine
//!
//! This crate implements the selection engine that decides which files
//! to include in an artifact and at what inclusion level.

pub mod scorer;
pub mod ranker;
pub mod budget;

pub use scorer::SelectionScorer;
pub use ranker::FileRanker;
pub use budget::BudgetFitter;
