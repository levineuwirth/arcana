//! [`CardRegistry`] — the static dispatch table for card behavior.
//!
//! Addendum Section 11 / Phase 1 Task #21. Depends on tasks 4
//! (objects), 13 (effects), 15 (triggers).
//!
//! # Model
//!
//! Every card known to the engine is registered as a
//! [`CardDefinition`]. The registry returns definitions by
//! [`CardId`] (for fast lookup at resolution) and by name (for deck
//! construction and debug output). The registry is **shared, never
//! cloned** — callbacks may be plain `fn` pointers because they
//! don't need to live inside `GameState`.
//!
//! # Ability taxonomy
//!
//! - [`SpellAbilityDef`] — the effect that fires when the spell
//!   resolves. Only instant and sorcery cards need one; permanent
//!   spells' only spell-ability is "this becomes a permanent".
//! - [`ActivatedAbilityDef`] — an ability on a permanent that the
//!   controller can activate at a cost (CR 602). Mana abilities
//!   (CR 605) are marked as such and don't use the stack.
//! - [`TriggeredAbilityDef`] (re-exported from
//!   [`crate::triggers`]) — "whenever / when / at" abilities.
//! - Static abilities aren't modeled yet — Phase 1 handles them
//!   via the layer system directly (see `layers.rs`).
//!
//! # Target rechecking
//!
//! The registry's [`SpellAbilityDef::target_requirements`] and
//! [`ActivatedAbilityDef::target_requirements`] feed both the
//! legal-action enumerator (Task #19) and the CR 608.2b recheck at
//! resolution (Task #20's `resolve_top_of_stack`). A spell with
//! no target requirements has an empty vector.

use crate::collections::HashMap;

use crate::effects::Effect;
use crate::mana::ManaCost;
use crate::objects::{Characteristics, ObjectId};
use crate::stack::StackEntry;
use crate::state::GameState;
use crate::targets::{TargetRequirement, TargetSelection};
use crate::triggers::TriggeredAbilityDef;
use crate::types::{CardId, CounterKind, PlayerId, SmallString, StringInterner};

// =============================================================================
// Effect function signatures
// =============================================================================

/// Produces the effect list for a resolving spell. Called by
/// [`crate::engine::step`] between popping the stack entry and
/// finalizing it.
pub type SpellEffectFn =
    fn(&GameState, &StackEntry, &CardRegistry) -> Vec<Effect>;

/// Produces the effect list for a resolving activated ability.
/// Non-mana activated abilities go on the stack and resolve through
/// this callback; mana abilities skip the stack and the engine
/// calls their effect fn directly.
pub type ActivatedEffectFn =
    fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>;

// =============================================================================
// ActivationContext
// =============================================================================

/// Parameters passed to an activated ability's effect function.
/// Mirrors the subset of [`crate::actions::Action::ActivateAbility`]
/// the effect cares about.
#[derive(Clone, Debug)]
pub struct ActivationContext {
    pub source: ObjectId,
    pub controller: PlayerId,
    pub ability_index: usize,
    pub targets: TargetSelection,
    pub x_value: Option<u32>,
}

// =============================================================================
// CardDefinition
// =============================================================================

/// Everything the engine needs to know about a card's printed form
/// and abilities. Ownership lives in [`CardRegistry`]; game objects
/// refer to definitions by [`CardId`].
#[derive(Clone, Debug)]
pub struct CardDefinition {
    /// Interned card name (`"Lightning Bolt"`, `"Mountain"`…).
    pub name: SmallString,
    /// Base characteristics — mana cost, type line, base P/T, colors.
    /// When a permanent enters the battlefield or a spell is cast,
    /// this is cloned into the [`GameObject`] as its starting point
    /// before the layer system applies.
    pub base_characteristics: Characteristics,
    /// Resolution behavior for instants and sorceries.
    ///
    /// `None` for permanent spells (creature/artifact/enchantment/
    /// planeswalker/battle/land cards). Their resolution is
    /// "becomes a permanent", handled by the stack-finalizer.
    pub spell_ability: Option<SpellAbilityDef>,
    pub activated_abilities: Vec<ActivatedAbilityDef>,
    pub triggered_abilities: Vec<TriggeredAbilityDef>,
    /// CR 121.6a "enters with" clauses printed on the card itself
    /// (not state-installed replacements from other permanents).
    /// The engine applies these during spell resolution, before
    /// `after_enter_battlefield` runs SBA, so 0/0 creatures with
    /// "enters with X +1/+1 counters" survive.
    ///
    /// Other-source ETB replacements (Hardened Scales, Doubling
    /// Season) live on `GameState::replacement_effects` and compose
    /// through `place_counters`.
    pub enters_with: Vec<EntersWithSpec>,
}

impl CardDefinition {
    /// Minimal constructor — pure-data cards (vanilla creatures,
    /// basic lands without printed abilities).
    pub fn new(name: SmallString, characteristics: Characteristics) -> Self {
        Self {
            name,
            base_characteristics: characteristics,
            spell_ability: None,
            activated_abilities: Vec::new(),
            triggered_abilities: Vec::new(),
            enters_with: Vec::new(),
        }
    }

    pub fn with_spell_ability(mut self, ability: SpellAbilityDef) -> Self {
        self.spell_ability = Some(ability);
        self
    }

    pub fn with_activated_ability(mut self, ability: ActivatedAbilityDef) -> Self {
        self.activated_abilities.push(ability);
        self
    }

    pub fn with_triggered_ability(mut self, ability: TriggeredAbilityDef) -> Self {
        self.triggered_abilities.push(ability);
        self
    }

    pub fn with_enters_with(mut self, spec: EntersWithSpec) -> Self {
        self.enters_with.push(spec);
        self
    }
}

// =============================================================================
// EntersWithSpec
// =============================================================================

/// CR 121.6a "this permanent enters the battlefield with …" clauses
/// printed on the card's face. Processed during resolution of the
/// spell that creates the permanent.
#[derive(Clone, Debug)]
pub enum EntersWithSpec {
    /// "CARDNAME enters with N [kind] counters on it." `count` is
    /// known at registration time (Primordial Hydra would not use
    /// this — it reads X). Routed through `place_counters`, so
    /// Hardened Scales-style modifiers compose.
    Counters { kind: CounterKind, count: u32 },
    /// "CARDNAME enters with X [kind] counters on it." Reads the
    /// cast's `x_value`; zero `x_value` (e.g. spells cast without
    /// X, or cast-from-free paths that didn't announce X) places
    /// zero counters. Walking Ballista, Hangarback Walker,
    /// Endless One.
    CountersFromX { kind: CounterKind },
    /// "CARDNAME enters with a [kind] counter for each card exiled
    /// with it." Reads the cast's `delve_count` (number of cards
    /// exiled to pay the delve cost). Murktide Regent is the
    /// canonical consumer; degrades to zero counters when the
    /// caster didn't delve.
    CountersFromDelveCount { kind: CounterKind },
    /// "CARDNAME enters the battlefield tapped." Tap-lands,
    /// Cultivator Colossus, etc. Applies after any counters but
    /// before summoning sickness is stamped.
    Tapped,
}

// =============================================================================
// SpellAbilityDef
// =============================================================================

/// The effect a spell produces when it resolves. One per card (most
/// spells have a single block of rules text; split cards and modal
/// spells encode their branches inside the effect function).
#[derive(Clone, Debug)]
pub struct SpellAbilityDef {
    /// Oracle-style rules text. Used for debug output and testing.
    pub text: String,
    /// Per-clause target requirements for **non-modal** spells. For
    /// modal spells this stays empty; the effective target list is
    /// derived from the chosen modes via [`effective_target_requirements`].
    pub target_requirements: Vec<TargetRequirement>,
    /// CR 700.2 — declaration side of modal spells. `None` for the
    /// common non-modal case. When `Some`, the spell requires the
    /// caster to pick `[min_modes, max_modes]` clauses at cast time;
    /// each clause carries its own targets, and the effective targets
    /// for the cast are the concatenation of chosen clauses' targets
    /// **in card order** (CR 700.2c), not selection order.
    ///
    /// Cost-contingent mode expansion (entwine, escalate) will need
    /// to be resolved at announce time from the base `ModalSpec` plus
    /// `additional_costs`. Phase 2-A treats this as a static spec;
    /// the name stays `modal` rather than `base_modal` until the
    /// first cost-contingent card lands and forces the distinction.
    pub modal: Option<ModalSpec>,
    /// Function that produces the effect list at resolution.
    pub effect: SpellEffectFn,
}

/// CR 700.2 — modal-spell declaration. Lives on [`SpellAbilityDef`].
///
/// The `min_modes` / `max_modes` shape supports "Choose one" (1/1),
/// "Choose two" (2/2), "Choose one or both" (1/2), and "Choose any
/// number" (0/N). Entwine / escalate are deferred — they need a
/// cost-contingent resolution of `max_modes` at cast time, which
/// Phase 2-A doesn't wire.
#[derive(Clone, Debug)]
pub struct ModalSpec {
    pub min_modes: usize,
    pub max_modes: usize,
    pub clauses: Vec<ModeClause>,
}

/// A single clause of a modal spell's "choose one — ; or ; or"
/// block. Targets declared on the clause apply only when that clause
/// is chosen.
#[derive(Clone, Debug)]
pub struct ModeClause {
    pub text: String,
    pub target_requirements: Vec<TargetRequirement>,
}

/// Effective target requirements for a spell given its chosen modes.
/// Non-modal spells ignore `modes` and return the flat list. Modal
/// spells concatenate the chosen clauses' requirements **in card
/// order** (CR 700.2c) — `ModeChoice::mode_indices` is kept sorted
/// ascending, so iterating it gives the canonical order directly.
///
/// Returns an owned `Vec` rather than a borrow so callers don't have
/// to juggle lifetimes across the modal / non-modal branch. The cost
/// is small — `TargetRequirement` is a handful of words. If this
/// ever shows up in a profile, revisit with a `Cow`.
///
/// DEBT: target enumeration across `C(N, k)` mode subsets × per-clause
/// targets is a Cartesian product that can blow up on many-target
/// modals (e.g. Aminatou's Augury, Kolaghan's Command). The dedup-by-
/// characteristic-equivalence helper used for delve applies here too
/// and should be reused when that regime is reached.
pub fn effective_target_requirements(
    ability: &SpellAbilityDef,
    modes: &[crate::stack::ModeChoice],
) -> Vec<TargetRequirement> {
    let Some(modal) = ability.modal.as_ref() else {
        return ability.target_requirements.clone();
    };
    let Some(choice) = modes.first() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for idx in &choice.mode_indices {
        if let Some(clause) = modal.clauses.get(*idx) {
            out.extend(clause.target_requirements.iter().cloned());
        }
    }
    out
}

// =============================================================================
// ActivatedAbilityDef
// =============================================================================

/// An ability of the form `cost: effect` on a permanent (CR 602).
/// Mana abilities (CR 605) set [`Self::is_mana_ability`] true — they
/// bypass the stack and resolve immediately during activation.
#[derive(Clone, Debug)]
pub struct ActivatedAbilityDef {
    pub text: String,
    pub cost: ActivationCost,
    pub target_requirements: Vec<TargetRequirement>,
    /// CR 605: mana ability iff (a) costs don't include targets,
    /// (b) effect produces mana, (c) not a loyalty ability. The
    /// engine trusts this flag rather than re-deriving it.
    pub is_mana_ability: bool,
    /// CR 606 — loyalty ability. Only the PW's controller may activate,
    /// only at sorcery speed with the stack empty, and only one per
    /// planeswalker per turn (tracked via
    /// [`crate::state::GameState::loyalty_activated_this_turn`]). The
    /// engine trusts this flag rather than re-deriving from the cost
    /// shape.
    pub is_loyalty_ability: bool,
    pub effect: ActivatedEffectFn,
}

/// Non-target, non-mana costs for activating an ability.
#[derive(Clone, Debug, Default)]
pub struct ActivationCost {
    pub mana_cost: ManaCost,
    /// CR 602.1b — `{T}` tap cost.
    pub tap: bool,
    /// CR 118.12 — "as an additional cost, sacrifice ~".
    pub sacrifice: bool,
    /// Life cost; 0 for no life payment.
    pub life: u32,
    /// "Remove N [kind] counters from ~: …" — the counter always
    /// comes off the ability's own source. Walking Ballista's
    /// remove-a-+1/+1-to-ping and planeswalker minus-loyalty costs
    /// both fit this shape. Legal-action enumeration filters the
    /// ability out when the source doesn't have enough counters.
    pub remove_self_counter: Option<(CounterKind, u32)>,
    /// "Put N [kind] counters on ~: …" — the mirror of
    /// [`Self::remove_self_counter`]. Planeswalker plus-loyalty
    /// costs are the canonical consumer (CR 606.2). Routed through
    /// [`GameState::place_counters`] so Hardened Scales / Doubling
    /// Season compose for `+1` activations just like they do for
    /// ETB counters.
    pub add_self_counter: Option<(CounterKind, u32)>,
}

impl ActivationCost {
    /// "{T}: …" — the bare tap cost common to basic lands' mana
    /// abilities.
    pub fn tap_only() -> Self {
        Self { tap: true, ..Self::default() }
    }

    /// "cost is free" — used for intrinsic abilities.
    pub fn free() -> Self { Self::default() }
}

// =============================================================================
// CardRegistry
// =============================================================================

/// Static dispatch table mapping [`CardId`] → [`CardDefinition`],
/// with a reverse index by interned name. Shared across all games —
/// never cloned.
///
/// Build the registry once at engine start, register every card
/// definition, then thread `&CardRegistry` through [`crate::engine::step`]
/// and [`crate::engine::new_game`]. Registration is not thread-safe;
/// populate the registry single-threaded before spawning game
/// workers.
#[derive(Debug, Default)]
pub struct CardRegistry {
    /// Monotonic allocator for fresh ids. `0` is reserved as an
    /// "unregistered" sentinel for test objects that don't go
    /// through the registry.
    next_card_id: CardId,
    definitions: HashMap<CardId, CardDefinition>,
    by_name: HashMap<SmallString, CardId>,
    /// Interner for card names, subtypes, and anywhere else the
    /// engine uses [`SmallString`]. Shared so inserting "Human" once
    /// in registration pays off for every Human card thereafter.
    interner: StringInterner,
}

impl CardRegistry {
    /// Build an empty registry with a fresh interner. `next_card_id`
    /// starts at 1 so `0` remains available as the unregistered
    /// sentinel.
    pub fn new() -> Self {
        Self {
            next_card_id: 1,
            definitions: HashMap::default(),
            by_name: HashMap::default(),
            interner: StringInterner::new(),
        }
    }

    /// Immutable access to the interner — e.g. for resolving a
    /// [`SmallString`] on a characteristic during debug formatting.
    pub fn interner(&self) -> &StringInterner { &self.interner }

    /// Mutable access, for registration code that needs to intern
    /// subtypes ("Mountain", "Bear") while building a card definition.
    pub fn interner_mut(&mut self) -> &mut StringInterner { &mut self.interner }

    /// Register a card. Returns the freshly assigned [`CardId`]. The
    /// card's name must be unique — a duplicate registration panics,
    /// which is the correct behavior for a programming bug.
    pub fn register(&mut self, definition: CardDefinition) -> CardId {
        let id = self.next_card_id;
        self.next_card_id = self.next_card_id.checked_add(1)
            .expect("CardRegistry: CardId counter overflow");
        let name = definition.name;
        if self.by_name.insert(name, id).is_some() {
            panic!("CardRegistry::register: duplicate name for CardId {id}");
        }
        self.definitions.insert(id, definition);
        id
    }

    /// Does this registry know about `card_id`? `CardId::0` (the
    /// unregistered sentinel) always returns `false`.
    pub fn contains(&self, card_id: CardId) -> bool {
        self.definitions.contains_key(&card_id)
    }

    pub fn get(&self, card_id: CardId) -> Option<&CardDefinition> {
        self.definitions.get(&card_id)
    }

    /// Look up a card by its already-interned name.
    pub fn get_by_name(&self, name: SmallString) -> Option<&CardDefinition> {
        self.by_name.get(&name)
            .and_then(|id| self.definitions.get(id))
    }

    /// Look up a card by its raw `&str` name. Returns `None` if the
    /// name isn't registered. Prefer [`Self::card_id_by_name`] +
    /// [`Self::get`] when building a deck so the id can be cached.
    pub fn get_by_str(&self, name: &str) -> Option<&CardDefinition> {
        let interned = self.interner.lookup(name)?;
        self.get_by_name(interned)
    }

    /// Look up the [`CardId`] for a card by its raw `&str` name.
    pub fn card_id_by_name(&self, name: &str) -> Option<CardId> {
        let interned = self.interner.lookup(name)?;
        self.by_name.get(&interned).copied()
    }

    /// Iterate every registered definition with its id. Useful for
    /// debug dumps and deck validation.
    pub fn iter(&self)
        -> impl Iterator<Item = (CardId, &CardDefinition)> + '_
    {
        self.definitions.iter().map(|(id, def)| (*id, def))
    }

    pub fn len(&self) -> usize { self.definitions.len() }
    pub fn is_empty(&self) -> bool { self.definitions.is_empty() }
}

// =============================================================================
// Deck construction helpers
// =============================================================================

/// Build a list of [`CardId`]s from a shorthand deck list of
/// `(name, count)` pairs. Panics if any name is missing from the
/// registry — the caller is expected to have registered the full
/// card pool first.
///
/// The returned ids come in deck-list order; [`crate::engine::new_game`]
/// shuffles them deterministically.
pub fn build_deck(entries: &[(&str, u32)], registry: &CardRegistry) -> Vec<CardId> {
    let mut deck = Vec::new();
    for (name, count) in entries {
        let id = registry.card_id_by_name(name).unwrap_or_else(||
            panic!("build_deck: card {name:?} not registered"));
        for _ in 0..*count {
            deck.push(id);
        }
    }
    deck
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::targets::TargetRequirement;
    use crate::types::{ColorSet, PtValue, TypeLine};
    use crate::events::DamageTarget;
    use crate::targets::TargetChoice;

    fn vanilla(name: &str, registry: &mut CardRegistry, p: i32, t: i32) -> CardId {
        let interned = registry.interner_mut().intern(name);
        registry.register(CardDefinition::new(interned, Characteristics {
            mana_cost: Some(ManaCost::parse("{1}{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }))
    }

    fn bolt_effect(
        _state: &GameState,
        entry: &StackEntry,
        _reg: &CardRegistry,
    ) -> Vec<Effect> {
        let target = entry.targets.targets.first().expect("Bolt has a target");
        let dt = match target {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                crate::targets::ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                crate::targets::ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        vec![Effect::DealDamage { source: entry.source, target: dt, amount: 3 }]
    }

    #[test]
    fn new_registry_allocates_ids_monotonically() {
        let mut r = CardRegistry::new();
        let a = vanilla("A", &mut r, 1, 1);
        let b = vanilla("B", &mut r, 2, 2);
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(r.len(), 2);
    }

    #[test]
    #[should_panic(expected = "duplicate name")]
    fn duplicate_registration_panics() {
        let mut r = CardRegistry::new();
        let _ = vanilla("DuplicateBear", &mut r, 1, 1);
        let _ = vanilla("DuplicateBear", &mut r, 2, 2);
    }

    #[test]
    fn lookup_by_str_roundtrips() {
        let mut r = CardRegistry::new();
        let a = vanilla("Grizzly Bears", &mut r, 2, 2);
        assert_eq!(r.card_id_by_name("Grizzly Bears"), Some(a));
        let def = r.get_by_str("Grizzly Bears").unwrap();
        assert_eq!(def.base_characteristics.power, Some(PtValue::Fixed(2)));
        assert!(r.get_by_str("Nonexistent").is_none());
    }

    #[test]
    fn unregistered_sentinel_is_zero() {
        let r = CardRegistry::new();
        assert!(!r.contains(0));
    }

    #[test]
    fn with_spell_ability_records_target_requirements() {
        let mut r = CardRegistry::new();
        let name = r.interner_mut().intern("Bolt");
        let def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Bolt deals 3 to any target".into(),
            target_requirements: vec![TargetRequirement::any_target()],
            modal: None,
            effect: bolt_effect,
        });
        let id = r.register(def);
        let looked_up = r.get(id).unwrap();
        assert_eq!(
            looked_up.spell_ability.as_ref().unwrap().target_requirements.len(),
            1,
        );
    }

    #[test]
    fn build_deck_honors_counts_and_ordering() {
        let mut r = CardRegistry::new();
        let _a = vanilla("BearA", &mut r, 2, 2);
        let _b = vanilla("BearB", &mut r, 3, 3);
        let deck = build_deck(&[("BearA", 3), ("BearB", 2)], &r);
        assert_eq!(deck.len(), 5);
        assert_eq!(deck[0], deck[1]);
        assert_eq!(deck[0], deck[2]);
        assert_ne!(deck[2], deck[3]);
    }

    #[test]
    #[should_panic(expected = "not registered")]
    fn build_deck_panics_on_missing_card() {
        let r = CardRegistry::new();
        let _ = build_deck(&[("Nowhere", 1)], &r);
    }

    #[test]
    fn activation_cost_helpers() {
        let tap = ActivationCost::tap_only();
        assert!(tap.tap);
        assert_eq!(tap.life, 0);
        assert!(!tap.sacrifice);

        let free = ActivationCost::free();
        assert!(!free.tap);
    }
}
