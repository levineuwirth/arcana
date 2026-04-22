//! Agentic card generation pipeline:
//!   - parses Scryfall bulk data
//!   - generates Rust source for each card
//!   - runs compilation and smoke tests
//!   - flags cards needing manual intervention

pub mod scryfall;
pub mod classifier;
pub mod prompt;
pub mod verify;
pub mod llm;
