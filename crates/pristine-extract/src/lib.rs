//! Pristine Extract - Content extraction
//!
//! This crate handles extracting content from files,
//! including encoding detection and text extraction.

pub mod extractor;
pub mod encoding;

pub use extractor::ContentExtractor;
