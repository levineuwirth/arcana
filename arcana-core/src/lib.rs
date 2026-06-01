pub mod collections;
pub mod types;
pub mod mana;
pub mod zones;
pub mod objects;
pub mod events;
pub mod state;
pub mod turn;
pub mod priority;
pub mod stack;
pub mod combat;
pub mod sba;
pub mod layers;
pub mod replacement;
pub mod triggers;
pub mod effects;
pub mod keywords;
pub mod targets;
pub mod script;
pub mod conditions;
pub mod dungeon;
pub mod behavioral;
pub mod actions;
pub mod format;
pub mod legal_actions;
pub mod registry;
pub mod engine;

// Re-export core types for convenience
pub use state::GameState;
pub use actions::Action;
pub use engine::{step, EngineYield};
pub use format::{FormatConfig, MulliganRule};
pub use objects::{ObjectId, ObjectArena, GameObject};
pub use registry::CardRegistry;
pub use types::*;

// Synonym module: card-gen agents frequently write
// `use arcana_core::counters::...` even though no such module exists
// canonically. This `counters` namespace re-surfaces the counter
// primitives so wrong-path imports still compile. Equivalent to
// importing `CounterKind` from `types::`.
pub mod counters {
    pub use crate::types::{CounterKind, CounterMap};
    pub use crate::types::CounterKind as CounterType;
}
