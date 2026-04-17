//! CardRegistry, registration, lookup.
//!
//! The registry owns the process's [`StringInterner`]: all card names,
//! subtypes, and other repeated strings are interned here at registration
//! time (startup, single-threaded). During gameplay, `GameState` carries
//! only `SmallString` handles, and parallel simulations can share an
//! `Arc<CardRegistry>` immutably without coordination.

use std::collections::HashMap;
use arcana_core::types::{CardId, SmallString, StringInterner};
use arcana_core::objects::Characteristics;
use arcana_core::effects::KeywordAbility;

#[derive(Default)]
pub struct CardRegistry {
    cards: HashMap<CardId, CardDefinition>,
    name_index: HashMap<String, CardId>,
    interner: StringInterner,
}

impl CardRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, def: CardDefinition) {
        self.name_index.insert(def.name.clone(), def.id);
        self.cards.insert(def.id, def);
    }

    pub fn get(&self, id: CardId) -> Option<&CardDefinition> {
        self.cards.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&CardDefinition> {
        self.name_index.get(name).and_then(|id| self.cards.get(id))
    }

    // --- Interner access ---

    /// Intern a string into this registry's interner. Used by card
    /// registration code to convert names / subtypes into `SmallString`
    /// handles.
    pub fn intern(&mut self, s: &str) -> SmallString {
        self.interner.intern(s)
    }

    /// Resolve a handle produced by this registry.
    pub fn resolve(&self, id: SmallString) -> Option<&str> {
        self.interner.resolve(id)
    }

    /// Borrow the interner for immutable operations (e.g. `SubtypeSet::
    /// contains_name`).
    pub fn interner(&self) -> &StringInterner { &self.interner }

    /// Borrow the interner mutably (for bulk registration, or
    /// `SubtypeSet::from_names`).
    pub fn interner_mut(&mut self) -> &mut StringInterner { &mut self.interner }
}

pub struct CardDefinition {
    pub id: CardId,
    pub name: String,
    pub base_characteristics: Characteristics,
    pub oracle_text: String,
    /// Spell abilities (resolve effects)
    pub spell_abilities: Vec<SpellAbilityDef>,
    /// Activated abilities
    pub activated_abilities: Vec<ActivatedAbilityDef>,
    /// Triggered abilities
    pub triggered_abilities: Vec<arcana_core::triggers::TriggeredAbilityDef>,
    /// Static abilities (continuous effects, replacement effects)
    pub static_abilities: Vec<StaticAbilityDef>,
    /// Keyword abilities
    pub keywords: Vec<KeywordAbility>,
    /// AI hints (optional, not used by engine)
    pub ai_hints: Option<AiHints>,
}

pub struct SpellAbilityDef {
    pub targets: Vec<arcana_core::targets::TargetRequirement>,
    /// fn pointer (not closure) so CardDefinition can be statically dispatched
    pub resolve: fn(
        &arcana_core::state::GameState,
        &arcana_core::stack::StackEntry,
        &CardRegistry,
    ) -> Vec<arcana_core::effects::Effect>,
}

pub struct ActivatedAbilityDef {
    pub cost: Vec<ActivationCost>,
    pub targets: Vec<arcana_core::targets::TargetRequirement>,
    pub resolve: fn(
        &arcana_core::state::GameState,
        &arcana_core::stack::StackEntry,
        &CardRegistry,
    ) -> Vec<arcana_core::effects::Effect>,
}

pub enum ActivationCost {
    Mana(arcana_core::mana::ManaCost),
    Tap,
    Untap,
    Sacrifice,
    PayLife(u32),
    Discard(u32),
    // ... extensible
}

pub struct StaticAbilityDef {
    // TODO: define when implementing layers
}

pub struct AiHints {
    // Optional metadata to help training (e.g. "this card is a removal spell")
}
