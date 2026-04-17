//! Card registry mapping CardId to CardDefinition.
//! Each card definition contains closures/function pointers for its effects,
//! triggers, static abilities, and replacement effects.
//!
//! Generated card code lives in src/generated/.

pub mod registry;
pub mod keywords;
pub mod tokens;
pub mod generated;

pub use registry::{CardRegistry, CardDefinition};
