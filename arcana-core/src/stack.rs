//! The stack: [`StackEntry`], cast-spell procedure (CR 601.2), and the
//! resolution pipeline (CR 608) with CR 608.2b target recheck plumbed
//! in.
//!
//! Addendum Section 6, Phase 1 Task #11. Depends on tasks 4 (objects),
//! 6 (state), 8 (actions), 9 (targets), 10 (priority).
//!
//! # Stack model
//!
//! The stack is a `Vec<StackEntry>` stored on [`GameState`]. Each entry
//! represents one spell or ability in mid-flight. The top of the stack
//! is the last element (LIFO).
//!
//! # Cast-spell pipeline (CR 601.2)
//!
//! Casting unfolds in six sub-steps. This module handles the ones that
//! mutate the stack or emit events; payment, target selection, and
//! mode/X choice all happen at the [`crate::actions::Action`] layer
//! before any helper here is called.
//!
//! ```text
//!   601.2a  Announce the spell  -> announce_spell_on_stack
//!   601.2b  Choose modes, X, targets, additional costs
//!                               -> encoded in Action::CastSpell
//!   601.2c  Pay costs           -> Task #12 (mana payment solver)
//!   601.2d  Legality check      -> caller (engine)
//!   601.2e  "The spell has been cast" -> emit_spell_cast
//!   601.2f  Resolution (later)  -> resolve_top / finalize_resolved_spell
//! ```
//!
//! # Resolution pipeline (CR 608)
//!
//! At resolution time the engine:
//!
//! 1. Takes the top-of-stack entry ([`GameState::pop_stack_entry`]).
//! 2. Rechecks each target against the spell's targeting requirements
//!    ([`recheck_stack_entry_targets`], CR 608.2b). If *every* target
//!    has become illegal and the spell had at least one target, the
//!    spell is countered by the rules and skips effect application
//!    ([`counter_resolved_spell`]).
//! 3. Otherwise executes the effect (the `CardRegistry` callback — not
//!    in Task #11 scope) and then finalizes the resolved spell or
//!    ability ([`finalize_resolved_spell`] /
//!    [`finalize_resolved_ability`]).
//!
//! # A note on object identity across zones
//!
//! CR 400.7 says that a card becomes a "new object" each time it
//! changes zones. For Phase 1 we keep the same [`ObjectId`] across the
//! hand → stack → graveyard/battlefield transitions; the
//! [`GameEvent::ZoneChange`] event carries both `object_id` and
//! `new_id` so the event stream is honest about the rule even when our
//! arena reuses the id. The `new_id == object_id` equivalence will be
//! revisited when triggers that care about the distinction land.

use serde::{Serialize, Deserialize};

use crate::events::{DamageTarget, GameEvent, MoveCause};
use crate::objects::{Characteristics, ObjectId};
use crate::state::GameState;
use crate::targets::{
    TargetLegality, TargetRequirement, TargetSelection,
    validate_targets_on_resolution,
};
use crate::types::*;
use crate::zones::Zone;

// =============================================================================
// StackEntry
// =============================================================================

/// One entry on the stack — either a spell mid-resolution or an
/// activated/triggered ability.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackEntry {
    /// [`ObjectId`] of the stack object. For spells this is the same id
    /// as the card being cast (CR 400.7 caveat — see module docs).
    pub id: ObjectId,
    /// The physical card or permanent that owns this entry. For spells
    /// this equals [`Self::id`]; for abilities it's the permanent that
    /// activated / triggered the ability.
    pub source: ObjectId,
    /// Who controls this stack object. Usually the caster / activator.
    pub controller: PlayerId,
    pub kind: StackEntryKind,
    pub targets: TargetSelection,
    /// Mode selections (one entry per modal clause); empty for
    /// non-modal spells and abilities.
    pub modes: Vec<ModeChoice>,
    /// Chosen value of `{X}` if the spell or cost had one.
    pub x_value: Option<u32>,
    /// Snapshot of [`crate::state::GameState::storm_count`] at the
    /// moment this entry was put on the stack via the *cast* path
    /// (CR 702.40a). Storm reads this directly — N copies = the
    /// snapshot value (= "spells cast before this one this turn").
    /// Copies pushed via [`crate::effects::Effect::CopySpell`] inherit
    /// the original's value but never trigger storm again because
    /// they don't go through [`GameState::announce_spell_on_stack`].
    /// Non-spell entries (activated/triggered abilities) leave this 0.
    pub storm_count_at_cast: u32,
    /// Snapshot of the spell's targeting clauses, populated by
    /// [`GameState::announce_spell_on_stack`] from the registry. Lets
    /// [`crate::effects::Effect::CopySpell`] (and storm copies) push
    /// a [`crate::actions::ChoiceKind::ChooseTargets`] without needing
    /// registry access at effect-execution time. Empty for spells with
    /// no targets and for non-spell entries.
    ///
    /// Skipped by serde: [`crate::targets::TargetRequirement`] carries
    /// fn-pointer filters and isn't serializable. A deserialized
    /// stack entry comes back with this empty — copies made of it
    /// won't push a target-choice prompt. Replay paths that copy
    /// spells across a serialize boundary need to re-derive from the
    /// registry; Phase 2 doesn't exercise that path.
    #[serde(skip)]
    pub target_requirements: Vec<crate::targets::TargetRequirement>,
}

impl StackEntry {
    /// Construct a new spell stack entry.
    pub fn new_spell(
        id: ObjectId,
        controller: PlayerId,
        card_id: CardId,
        characteristics: Characteristics,
        targets: TargetSelection,
        modes: Vec<ModeChoice>,
        x_value: Option<u32>,
    ) -> Self {
        Self {
            id,
            source: id,
            controller,
            kind: StackEntryKind::Spell { card_id, characteristics },
            targets,
            modes,
            x_value,
            // Caller (announce_spell_on_stack) overwrites with the
            // pre-cast snapshot of state.storm_count.
            storm_count_at_cast: 0,
            // Caller (announce_spell_on_stack) populates from registry.
            target_requirements: Vec::new(),
        }
    }

    /// Construct a new activated-ability stack entry.
    pub fn new_activated_ability(
        id: ObjectId,
        source: ObjectId,
        controller: PlayerId,
        ability_id: AbilityId,
        text: String,
        targets: TargetSelection,
        modes: Vec<ModeChoice>,
        x_value: Option<u32>,
    ) -> Self {
        Self {
            id,
            source,
            controller,
            kind: StackEntryKind::ActivatedAbility { ability_id, text },
            targets,
            modes,
            x_value,
            storm_count_at_cast: 0,
            target_requirements: Vec::new(),
        }
    }

    /// Construct a new triggered-ability stack entry.
    pub fn new_triggered_ability(
        id: ObjectId,
        source: ObjectId,
        controller: PlayerId,
        trigger_id: TriggerId,
        trigger_event: GameEvent,
        text: String,
        targets: TargetSelection,
        modes: Vec<ModeChoice>,
    ) -> Self {
        Self {
            id,
            source,
            controller,
            kind: StackEntryKind::TriggeredAbility {
                trigger_id,
                trigger_event,
                text,
            },
            targets,
            modes,
            x_value: None,
            storm_count_at_cast: 0,
            target_requirements: Vec::new(),
        }
    }

    pub fn is_spell(&self) -> bool {
        matches!(self.kind, StackEntryKind::Spell { .. })
    }

    pub fn is_ability(&self) -> bool { !self.is_spell() }

    pub fn is_activated(&self) -> bool {
        matches!(self.kind, StackEntryKind::ActivatedAbility { .. })
    }

    pub fn is_triggered(&self) -> bool {
        matches!(self.kind, StackEntryKind::TriggeredAbility { .. })
    }

    /// The card id of a spell entry. `None` for abilities.
    pub fn card_id(&self) -> Option<CardId> {
        match &self.kind {
            StackEntryKind::Spell { card_id, .. } => Some(*card_id),
            _ => None,
        }
    }

    /// The characteristics of the stack-spell — post-replacement,
    /// post-layer copy taken when the spell was cast. `None` for
    /// abilities.
    pub fn characteristics(&self) -> Option<&Characteristics> {
        match &self.kind {
            StackEntryKind::Spell { characteristics, .. } => Some(characteristics),
            _ => None,
        }
    }

    /// True when this entry had at least one target chosen. Used by
    /// CR 608.2b: the "all targets illegal ⇒ countered" rule only
    /// applies when at least one target was chosen.
    pub fn has_targets(&self) -> bool { !self.targets.targets.is_empty() }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StackEntryKind {
    Spell {
        card_id: CardId,
        /// Snapshot of the spell's characteristics at cast time. The
        /// snapshot is what CR 112.2a's "characteristics of a spell
        /// while on the stack" refers to; effects that would modify
        /// the card mid-resolution operate on this copy.
        characteristics: Characteristics,
    },
    ActivatedAbility {
        ability_id: AbilityId,
        /// Oracle-text snippet for logging / debug; the actual effect
        /// dispatch goes through `CardRegistry`.
        text: String,
    },
    TriggeredAbility {
        trigger_id: TriggerId,
        /// The event that caused this trigger to enter the stack. Kept
        /// around so the trigger's effect fn has the context it needs.
        trigger_event: GameEvent,
        text: String,
    },
}

// =============================================================================
// ModeChoice
// =============================================================================

/// A single modal-choice payload. For a spell like "Choose two —",
/// this is the indices of the chosen modes within the card's modal
/// clause list.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeChoice {
    pub mode_indices: Vec<usize>,
}

impl ModeChoice {
    pub fn new(mode_indices: Vec<usize>) -> Self { Self { mode_indices } }
    pub fn empty() -> Self { Self::default() }
    pub fn is_empty(&self) -> bool { self.mode_indices.is_empty() }
    pub fn len(&self) -> usize { self.mode_indices.len() }
}

// =============================================================================
// ResolutionOutcome — what happened when a stack entry resolved
// =============================================================================

/// Reports what happened as a stack entry resolved. Returned by
/// [`GameState::recheck_and_classify_resolution`], which encapsulates
/// the CR 608.2b target recheck.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolutionOutcome {
    /// The spell or ability resolves normally. For multi-target
    /// effects, the vector records per-target legality so the effect
    /// application can skip individual illegal targets (CR 608.2b).
    Resolve { target_legality: Vec<TargetLegality> },
    /// Every chosen target has become illegal — per CR 608.2b the
    /// spell or ability is countered by the rules and no effect
    /// applies. The caller should route the spell card (if any) to
    /// the graveyard via [`GameState::counter_resolved_spell`].
    CounteredIllegalTargets,
}

// =============================================================================
// Stack manipulation on GameState
// =============================================================================

impl GameState {
    // --- basic stack queries ------------------------------------------------

    pub fn stack_size(&self) -> usize { self.stack.len() }

    pub fn stack_is_empty(&self) -> bool { self.stack.is_empty() }

    /// Top-of-stack — the entry that would resolve next.
    pub fn top_of_stack(&self) -> Option<&StackEntry> {
        self.stack.last()
    }

    pub fn top_of_stack_mut(&mut self) -> Option<&mut StackEntry> {
        self.stack.last_mut()
    }

    /// All stack entries, bottom to top.
    pub fn stack_entries(&self) -> &[StackEntry] { &self.stack }

    /// Look up a stack entry by id (not position — the stack may
    /// contain abilities whose ids are freshly allocated).
    pub fn find_stack_entry(&self, id: ObjectId) -> Option<&StackEntry> {
        self.stack.iter().find(|e| e.id == id)
    }

    pub fn find_stack_entry_mut(&mut self, id: ObjectId) -> Option<&mut StackEntry> {
        self.stack.iter_mut().find(|e| e.id == id)
    }

    /// Position of a stack entry by id; 0 = bottom, `stack_size() - 1`
    /// = top.
    pub fn stack_position_of(&self, id: ObjectId) -> Option<usize> {
        self.stack.iter().position(|e| e.id == id)
    }

    // --- push / pop ---------------------------------------------------------

    /// Push a pre-built stack entry onto the stack. Does not emit any
    /// events — callers using the higher-level [`announce_spell_on_stack`]
    /// or the ability-activation helpers (Task #14) handle events
    /// themselves. Exposed for tests and for low-level engine code.
    ///
    /// [`announce_spell_on_stack`]: Self::announce_spell_on_stack
    pub fn push_stack_entry(&mut self, entry: StackEntry) {
        self.stack.push(entry);
    }

    /// Pop the top of stack. The caller is expected to either finalize
    /// the resolved spell/ability ([`Self::finalize_resolved_spell`] /
    /// [`Self::finalize_resolved_ability`]) or counter it
    /// ([`Self::counter_resolved_spell`]).
    pub fn pop_stack_entry(&mut self) -> Option<StackEntry> {
        self.stack.pop()
    }

    // --- CR 601.2a: announce spell on stack --------------------------------

    /// CR 601.2a — move `object_id` from its current zone onto the
    /// stack and allocate a [`StackEntry`]. Emits
    /// [`GameEvent::ZoneChange`] with cause [`MoveCause::Cast`].
    ///
    /// Does **not** emit [`GameEvent::SpellCast`] — that fires at
    /// CR 601.2e via [`Self::emit_spell_cast`] once costs and choices
    /// are all settled.
    ///
    /// Returns the id of the stack entry created (equal to
    /// `object_id` for Phase 1 — see module doc on CR 400.7).
    ///
    /// # Panics
    /// If `object_id` does not exist in the arena, or its
    /// characteristics don't represent a castable spell (no mana cost
    /// is fine; a token or emblem is not). Programmer bug.
    pub fn announce_spell_on_stack(
        &mut self,
        object_id: ObjectId,
        controller: PlayerId,
        targets: TargetSelection,
        modes: Vec<ModeChoice>,
        x_value: Option<u32>,
        target_requirements: Vec<crate::targets::TargetRequirement>,
    ) -> ObjectId {
        if !self.objects.contains(object_id) {
            panic!("announce_spell_on_stack: object {object_id} not in arena");
        }
        let (new_id, from) = self.swap_to_zone_reid(object_id, Zone::Stack)
            .expect("announce_spell_on_stack: swap_to_zone_reid returned None");

        // Caster controls the spell while it's on the stack.
        let (card_id, characteristics) = {
            let obj = self.objects.get_mut(new_id).unwrap();
            obj.controller = controller;
            (obj.card_id, obj.characteristics.clone())
        };

        let mut entry = StackEntry::new_spell(
            new_id, controller, card_id, characteristics,
            targets, modes, x_value,
        );
        // CR 702.40a: snapshot BEFORE incrementing — N copies for storm
        // = "spells cast before this one this turn".
        entry.storm_count_at_cast = self.storm_count;
        self.storm_count = self.storm_count.saturating_add(1);
        // Snapshot of the spell's targeting clauses so copies (storm,
        // CopySpell) can push ChooseTargets without registry access.
        entry.target_requirements = target_requirements;
        self.stack.push(entry);

        self.emit(GameEvent::ZoneChange {
            object_id,
            from,
            to: Zone::Stack,
            new_id,
            cause: MoveCause::Cast,
        });

        new_id
    }

    /// CR 601.2e — the spell has been cast. Emits
    /// [`GameEvent::SpellCast`]. Call this once the cast procedure
    /// completes (costs paid, all choices made). Trigger matching
    /// picks it up from the event log.
    pub fn emit_spell_cast(&mut self, stack_entry_id: ObjectId) {
        let (card_id, controller, targets) = {
            let entry = self.find_stack_entry(stack_entry_id).unwrap_or_else(||
                panic!("emit_spell_cast: no stack entry {stack_entry_id}"));
            let card_id = entry.card_id().unwrap_or_else(||
                panic!("emit_spell_cast: stack entry {stack_entry_id} is not a spell"));
            (card_id, entry.controller, entry.targets.clone())
        };
        self.emit(GameEvent::SpellCast {
            object_id: stack_entry_id,
            card_id,
            controller,
            targets,
        });
    }

    // --- CR 608.2b: target recheck -----------------------------------------

    /// CR 608.2b — recheck every target on the stack entry against the
    /// supplied `requirements`. One requirement per targeting clause
    /// in the card text, in the same order as the `targets` vector
    /// was laid down at cast time.
    ///
    /// Returns [`ResolutionOutcome::CounteredIllegalTargets`] iff the
    /// entry had at least one chosen target and *every* target is
    /// illegal — this is the "countered by the rules" case.
    /// Otherwise returns [`ResolutionOutcome::Resolve`] with a
    /// per-target legality vector the effect dispatcher can use to
    /// skip individual illegal targets.
    ///
    /// `source_controller` disambiguates "you" / "opponent" in the
    /// target filter; it's the stack entry's controller.
    ///
    /// When the number of chosen targets differs from the number of
    /// requirements (a malformed selection), this treats every
    /// "extra" target as illegal so the caller is forced to notice.
    pub fn recheck_and_classify_resolution(
        &self,
        entry: &StackEntry,
        requirements: &[TargetRequirement],
    ) -> ResolutionOutcome {
        let chosen = &entry.targets.targets;

        // No-target spell/ability always resolves normally.
        if chosen.is_empty() {
            return ResolutionOutcome::Resolve { target_legality: Vec::new() };
        }

        // Recheck each chosen target against its corresponding
        // requirement by position. Extra chosen targets (more than the
        // requirement count) are treated as illegal.
        let mut legality = Vec::with_capacity(chosen.len());
        for (i, choice) in chosen.iter().enumerate() {
            let legal = match requirements.get(i) {
                Some(req) => req.matches_choice(choice, self, entry.controller),
                None => false,
            };
            legality.push(if legal { TargetLegality::Legal } else { TargetLegality::Illegal });
        }

        if legality.iter().all(|l| *l == TargetLegality::Illegal) {
            ResolutionOutcome::CounteredIllegalTargets
        } else {
            ResolutionOutcome::Resolve { target_legality: legality }
        }
    }

    // --- CR 608.2d: finalize a resolved spell ------------------------------

    /// Finalize a spell whose effect has been applied. Instants and
    /// sorceries move to their owner's graveyard; permanent-type
    /// spells enter the battlefield. Emits the appropriate zone-change
    /// events and [`GameEvent::SpellResolved`].
    ///
    /// Per CR 400.7 the object is re-id'd on the stack-to-destination
    /// transition: LKI is snapshotted under the old id, a fresh id is
    /// allocated, and the arriving-side events
    /// (`EntersBattlefield::object_id`, `SpellResolved::object_id`,
    /// `PutIntoGraveyard::object_id`, `ZoneChange::new_id`) carry the
    /// new id. `ZoneChange::object_id` still carries the old (stack)
    /// id for symmetry with the leaving side.
    ///
    /// Takes the stack entry by value — the caller is expected to have
    /// already popped it via [`Self::pop_stack_entry`].
    pub fn finalize_resolved_spell(&mut self, entry: StackEntry) {
        let StackEntry { id, controller, .. } = entry;
        let chars = entry.characteristics().cloned().unwrap_or_else(||
            panic!("finalize_resolved_spell: entry {id} is not a spell"));
        let owner = self.objects.get(id)
            .unwrap_or_else(|| panic!(
                "finalize_resolved_spell: object {id} vanished from arena"))
            .owner;
        let destination = if chars.is_permanent() {
            Zone::Battlefield
        } else {
            Zone::Graveyard(owner)
        };

        let (new_id, from) = self.swap_to_zone_reid(id, destination)
            .expect("finalize_resolved_spell: swap_to_zone_reid returned None");

        // For battlefield entries the caster becomes the controller
        // (CR 110.2a). Set it before the ETB hook runs so replacement
        // effects that key off "controlled by you" see the right
        // player.
        if destination == Zone::Battlefield {
            if let Some(obj) = self.objects.get_mut(new_id) {
                obj.controller = controller;
            }
        }

        self.emit(GameEvent::ZoneChange {
            object_id: id,
            from,
            to: destination,
            new_id,
            cause: MoveCause::SpellResolution,
        });

        if destination == Zone::Battlefield {
            self.after_enter_battlefield(new_id);
            self.emit(GameEvent::EntersBattlefield {
                object_id: new_id,
                from_zone: from,
                was_cast: true,
            });
        } else if matches!(destination, Zone::Graveyard(_)) {
            self.emit(GameEvent::PutIntoGraveyard {
                object_id: new_id,
                from,
            });
        }

        // `SpellResolved` references the stack-entry id so consumers
        // can correlate it with the preceding `SpellCast` event.
        self.emit(GameEvent::SpellResolved { object_id: id });
    }

    /// Finalize a resolved activated or triggered ability. Abilities
    /// don't have a card to route anywhere — they just emit
    /// [`GameEvent::AbilityResolved`] and disappear.
    pub fn finalize_resolved_ability(&mut self, entry: StackEntry) {
        assert!(entry.is_ability(),
            "finalize_resolved_ability called on a spell entry");
        self.emit(GameEvent::AbilityResolved { object_id: entry.id });
    }

    // --- Countering --------------------------------------------------------

    /// Counter a resolving spell per CR 608.2b — move the card to its
    /// owner's graveyard and emit [`GameEvent::SpellCountered`].
    ///
    /// Use this when [`Self::recheck_and_classify_resolution`]
    /// returned [`ResolutionOutcome::CounteredIllegalTargets`], or
    /// when a Counterspell-type effect resolves against this entry.
    ///
    /// Applies the CR 400.7 re-id on the stack-to-graveyard move: LKI
    /// is snapshotted under the old stack id, a fresh id is allocated
    /// for the graveyard object, and the events carry ids accordingly
    /// (see [`Self::finalize_resolved_spell`] for the pattern).
    pub fn counter_resolved_spell(&mut self, entry: StackEntry) {
        assert!(entry.is_spell(), "counter_resolved_spell requires a spell entry");
        let id = entry.id;
        let owner = self.objects.get(id)
            .unwrap_or_else(|| panic!(
                "counter_resolved_spell: object {id} vanished from arena"))
            .owner;
        let destination = Zone::Graveyard(owner);

        let (new_id, from) = self.swap_to_zone_reid(id, destination)
            .expect("counter_resolved_spell: swap_to_zone_reid returned None");

        self.emit(GameEvent::ZoneChange {
            object_id: id,
            from,
            to: destination,
            new_id,
            cause: MoveCause::SpellResolution,
        });
        self.emit(GameEvent::PutIntoGraveyard { object_id: new_id, from });
        self.emit(GameEvent::SpellCountered { object_id: id });
    }

    /// Counter an ability — remove it from the stack, emit
    /// [`GameEvent::SpellCountered`] using the ability's id. Abilities
    /// have no card to route anywhere.
    ///
    /// Pass the entry removed via [`Self::remove_stack_entry_by_id`].
    pub fn counter_resolved_ability(&mut self, entry: StackEntry) {
        assert!(entry.is_ability(),
            "counter_resolved_ability requires an ability entry");
        self.emit(GameEvent::SpellCountered { object_id: entry.id });
    }

    /// Remove a stack entry by id without finalizing it. Used by
    /// Counterspell-type effects that target a mid-flight spell
    /// (which may not be at the top of the stack).
    pub fn remove_stack_entry_by_id(&mut self, id: ObjectId) -> Option<StackEntry> {
        let pos = self.stack_position_of(id)?;
        Some(self.stack.remove(pos))
    }
}

// =============================================================================
// Free-function helpers
// =============================================================================

/// Convenience: recheck a stack entry's targets without classifying
/// into a [`ResolutionOutcome`]. Returns one [`TargetLegality`] per
/// chosen target, delegating to [`validate_targets_on_resolution`] for
/// each (requirement, choice) pair.
///
/// Extra chosen targets beyond the requirement count are reported as
/// [`TargetLegality::Illegal`].
pub fn recheck_stack_entry_targets(
    entry: &StackEntry,
    requirements: &[TargetRequirement],
    state: &GameState,
) -> Vec<TargetLegality> {
    // Walk requirements in lockstep with choices, slicing each choice
    // into a single-element TargetSelection so we can reuse the
    // library helper.
    entry.targets.targets.iter().enumerate().map(|(i, choice)| {
        let req = match requirements.get(i) {
            Some(r) => r,
            None => return TargetLegality::Illegal,
        };
        let single = TargetSelection { targets: vec![choice.clone()] };
        let legalities = validate_targets_on_resolution(
            req, &single, state, entry.controller);
        legalities.into_iter().next().unwrap_or(TargetLegality::Illegal)
    }).collect()
}

/// Convenience: would this entry's targets cause it to be countered
/// by CR 608.2b at resolution time? `true` iff the entry had at least
/// one target and none are still legal.
pub fn would_be_countered_for_targets(
    entry: &StackEntry,
    requirements: &[TargetRequirement],
    state: &GameState,
) -> bool {
    if !entry.has_targets() {
        return false;
    }
    // Use the per-choice recheck so a nonzero-length mismatch still
    // triggers "all illegal".
    let legs = recheck_stack_entry_targets(entry, requirements, state);
    !legs.is_empty() && legs.iter().all(|l| *l == TargetLegality::Illegal)
}

/// Not part of Task #11 scope, but handy: "would the spell countered
/// by the rules" matches the [`should_counter_due_to_illegal_targets`]
/// single-requirement helper. This re-export makes the link explicit.
pub use crate::targets::should_counter_due_to_illegal_targets as single_req_should_counter;

// =============================================================================
// A non-mutating "damage the stack entry pointed to by id" shortcut
// =============================================================================
//
// Kept tiny so tests and future code don't reinvent it. The
// stack entry's id is the target for Counter spells, Copy spells, and
// a few others; `DamageTarget` refers only to objects/players and is
// orthogonal. Exposed here so callers can ask "is this DamageTarget
// pointing at a stack entry?" without reaching into GameState.

pub fn damage_target_is_stack_entry(target: DamageTarget, state: &GameState) -> bool {
    match target {
        DamageTarget::Object(id) => state.find_stack_entry(id).is_some(),
        DamageTarget::Player(_) => false,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::objects::GameObject;
    use crate::targets::{
        ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    };

    // --- helpers -------------------------------------------------------------

    fn instant_chars() -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }
    }

    fn creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    fn put_object(
        state: &mut GameState,
        owner: PlayerId,
        zone: Zone,
        chars: Characteristics,
    ) -> ObjectId {
        let id = state.allocate_object_id();
        state.objects.insert(GameObject::new(id, owner, zone, /*card_id=*/ 1, chars));
        id
    }

    fn one_creature_target(c: ObjectId) -> TargetSelection {
        TargetSelection { targets: vec![TargetChoice::Object(c)] }
    }

    fn target_creature() -> TargetRequirement { TargetRequirement::target_creature() }

    // --- StackEntry helpers -------------------------------------------------

    #[test]
    fn stack_entry_kind_predicates() {
        let spell = StackEntry::new_spell(
            1, 0, /*card=*/ 1, instant_chars(),
            TargetSelection::new(), vec![], None,
        );
        let act = StackEntry::new_activated_ability(
            2, 10, 0, /*ability=*/ 1, "T: deal 1".into(),
            TargetSelection::new(), vec![], None,
        );
        let trg = StackEntry::new_triggered_ability(
            3, 10, 0, /*trigger=*/ 1,
            GameEvent::TurnBegins { player: 0, turn_number: 1 },
            "at upkeep...".into(),
            TargetSelection::new(), vec![],
        );

        assert!(spell.is_spell());
        assert!(!spell.is_ability());
        assert!(act.is_ability() && act.is_activated() && !act.is_triggered());
        assert!(trg.is_ability() && trg.is_triggered() && !trg.is_activated());

        assert_eq!(spell.card_id(), Some(1));
        assert!(act.card_id().is_none());
        assert!(trg.card_id().is_none());

        assert!(spell.characteristics().is_some());
        assert!(act.characteristics().is_none());
    }

    #[test]
    fn stack_entry_has_targets_empty_or_not() {
        let no_targets = StackEntry::new_spell(
            1, 0, 1, instant_chars(), TargetSelection::new(), vec![], None);
        assert!(!no_targets.has_targets());

        let with = StackEntry::new_spell(
            1, 0, 1, instant_chars(),
            TargetSelection { targets: vec![TargetChoice::Player(0)] },
            vec![], None);
        assert!(with.has_targets());
    }

    // --- push / pop / peek --------------------------------------------------

    #[test]
    fn push_and_pop_lifo() {
        let mut s = GameState::new(2, 0);
        assert!(s.stack_is_empty());

        s.push_stack_entry(StackEntry::new_spell(
            1, 0, 1, instant_chars(), TargetSelection::new(), vec![], None));
        s.push_stack_entry(StackEntry::new_spell(
            2, 0, 1, instant_chars(), TargetSelection::new(), vec![], None));
        assert_eq!(s.stack_size(), 2);

        assert_eq!(s.top_of_stack().map(|e| e.id), Some(2));
        let popped = s.pop_stack_entry().unwrap();
        assert_eq!(popped.id, 2);
        assert_eq!(s.top_of_stack().map(|e| e.id), Some(1));

        assert!(s.pop_stack_entry().is_some());
        assert!(s.stack_is_empty());
        assert!(s.pop_stack_entry().is_none());
    }

    #[test]
    fn find_and_position_by_id() {
        let mut s = GameState::new(2, 0);
        s.push_stack_entry(StackEntry::new_spell(
            10, 0, 1, instant_chars(), TargetSelection::new(), vec![], None));
        s.push_stack_entry(StackEntry::new_spell(
            20, 0, 1, instant_chars(), TargetSelection::new(), vec![], None));
        s.push_stack_entry(StackEntry::new_spell(
            30, 0, 1, instant_chars(), TargetSelection::new(), vec![], None));

        assert!(s.find_stack_entry(10).is_some());
        assert!(s.find_stack_entry(99).is_none());
        assert_eq!(s.stack_position_of(10), Some(0));
        assert_eq!(s.stack_position_of(30), Some(2));
        assert_eq!(s.stack_position_of(99), None);
    }

    // --- CR 601.2a: announce ------------------------------------------------

    #[test]
    fn announce_moves_card_to_stack_and_emits_zone_change() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        assert_eq!(s.objects.get(card).unwrap().zone, Zone::Hand(0));

        let id = s.announce_spell_on_stack(
            card, /*controller=*/ 0,
            TargetSelection::new(), vec![], None, vec![]);
        // CR 400.7: the card becomes a new object on the stack.
        assert_ne!(id, card);
        assert!(s.objects.get(card).is_none(), "old id must be gone from arena");
        assert_eq!(s.objects.get(id).unwrap().zone, Zone::Stack);
        assert_eq!(s.top_of_stack().map(|e| e.id), Some(id));

        let zone_change_emitted = s.event_log.iter().any(|ev| matches!(
            ev,
            GameEvent::ZoneChange { object_id, from: Zone::Hand(0), to: Zone::Stack, new_id, cause: MoveCause::Cast }
                if *object_id == card && *new_id == id
        ));
        assert!(zone_change_emitted, "announce should emit Hand→Stack ZoneChange(Cast) with old→new id");
    }

    #[test]
    fn announce_sets_controller_on_arena_object() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        // Pretend someone else (player 1) is casting — Yesterlock / control-steal.
        let id = s.announce_spell_on_stack(
            card, 1, TargetSelection::new(), vec![], None, vec![]);
        assert_eq!(s.objects.get(id).unwrap().controller, 1);
        assert_eq!(s.top_of_stack().unwrap().controller, 1);
    }

    #[test]
    #[should_panic(expected = "not in arena")]
    fn announce_missing_object_panics() {
        let mut s = GameState::new(2, 0);
        s.announce_spell_on_stack(999, 0, TargetSelection::new(), vec![], None, vec![]);
    }

    // --- CR 601.2e: SpellCast event ----------------------------------------

    #[test]
    fn emit_spell_cast_writes_event() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);

        let before = s.event_log.len();
        s.emit_spell_cast(stack_id);
        assert_eq!(s.event_log.len(), before + 1);
        assert!(matches!(
            s.event_log.last().unwrap(),
            GameEvent::SpellCast { object_id, .. } if *object_id == stack_id
        ));
    }

    #[test]
    #[should_panic(expected = "is not a spell")]
    fn emit_spell_cast_panics_on_ability_entry() {
        let mut s = GameState::new(2, 0);
        s.push_stack_entry(StackEntry::new_activated_ability(
            7, 10, 0, 1, "".into(), TargetSelection::new(), vec![], None));
        s.emit_spell_cast(7);
    }

    // --- CR 608.2b recheck --------------------------------------------------

    #[test]
    fn recheck_classifies_all_legal_as_resolve() {
        let mut s = GameState::new(2, 0);
        let c = put_object(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        let spell = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        s.announce_spell_on_stack(
            spell, 0, one_creature_target(c), vec![], None, vec![]);

        let entry = s.top_of_stack().unwrap().clone();
        let out = s.recheck_and_classify_resolution(&entry, &[target_creature()]);
        match out {
            ResolutionOutcome::Resolve { target_legality } => {
                assert_eq!(target_legality, vec![TargetLegality::Legal]);
            }
            _ => panic!("expected Resolve"),
        }
    }

    #[test]
    fn recheck_classifies_all_illegal_as_countered() {
        // Bolt a creature, creature leaves battlefield → countered.
        let mut s = GameState::new(2, 0);
        let c = put_object(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        let spell = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        s.announce_spell_on_stack(
            spell, 0, one_creature_target(c), vec![], None, vec![]);

        // Creature leaves the battlefield mid-stack.
        s.objects.get_mut(c).unwrap().zone = Zone::Exile;

        let entry = s.top_of_stack().unwrap().clone();
        let out = s.recheck_and_classify_resolution(&entry, &[target_creature()]);
        assert!(matches!(out, ResolutionOutcome::CounteredIllegalTargets));
    }

    #[test]
    fn recheck_no_targets_is_resolve() {
        let mut s = GameState::new(2, 0);
        s.push_stack_entry(StackEntry::new_spell(
            1, 0, 1, instant_chars(), TargetSelection::new(), vec![], None));
        let entry = s.top_of_stack().unwrap().clone();
        let out = s.recheck_and_classify_resolution(&entry, &[]);
        match out {
            ResolutionOutcome::Resolve { target_legality } => {
                assert!(target_legality.is_empty());
            }
            _ => panic!("expected Resolve"),
        }
    }

    #[test]
    fn recheck_mixed_reports_per_target() {
        let mut s = GameState::new(2, 0);
        let a = put_object(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        let b = put_object(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.push_stack_entry(StackEntry::new_spell(
            99, 0, 1, instant_chars(),
            TargetSelection { targets: vec![TargetChoice::Object(a), TargetChoice::Object(b)] },
            vec![], None,
        ));
        // Only b leaves.
        s.objects.get_mut(b).unwrap().zone = Zone::Graveyard(0);
        let entry = s.top_of_stack().unwrap().clone();
        let out = s.recheck_and_classify_resolution(
            &entry, &[target_creature(), target_creature()]);
        match out {
            ResolutionOutcome::Resolve { target_legality } => {
                assert_eq!(
                    target_legality,
                    vec![TargetLegality::Legal, TargetLegality::Illegal],
                );
            }
            _ => panic!("expected Resolve"),
        }
    }

    #[test]
    fn recheck_more_choices_than_requirements_is_illegal_tail() {
        let mut s = GameState::new(2, 0);
        let c = put_object(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.push_stack_entry(StackEntry::new_spell(
            99, 0, 1, instant_chars(),
            TargetSelection { targets: vec![TargetChoice::Object(c), TargetChoice::Object(c)] },
            vec![], None,
        ));
        let entry = s.top_of_stack().unwrap().clone();
        let out = s.recheck_and_classify_resolution(&entry, &[target_creature()]);
        // Second choice has no requirement -> illegal. First is legal.
        match out {
            ResolutionOutcome::Resolve { target_legality } => {
                assert_eq!(
                    target_legality,
                    vec![TargetLegality::Legal, TargetLegality::Illegal],
                );
            }
            _ => panic!("expected Resolve"),
        }
    }

    #[test]
    fn recheck_free_function_matches_method() {
        let mut s = GameState::new(2, 0);
        let c = put_object(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.push_stack_entry(StackEntry::new_spell(
            99, 0, 1, instant_chars(), one_creature_target(c), vec![], None));
        let entry = s.top_of_stack().unwrap().clone();
        let reqs = [target_creature()];

        let legs = recheck_stack_entry_targets(&entry, &reqs, &s);
        assert_eq!(legs, vec![TargetLegality::Legal]);
        assert!(!would_be_countered_for_targets(&entry, &reqs, &s));

        s.objects.get_mut(c).unwrap().zone = Zone::Exile;
        let legs = recheck_stack_entry_targets(&entry, &reqs, &s);
        assert_eq!(legs, vec![TargetLegality::Illegal]);
        assert!(would_be_countered_for_targets(&entry, &reqs, &s));
    }

    // --- Finalize resolved spell -------------------------------------------

    #[test]
    fn finalize_instant_goes_to_owner_graveyard() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        let entry = s.pop_stack_entry().unwrap();

        s.finalize_resolved_spell(entry);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|ev|
            matches!(ev, GameEvent::SpellResolved { object_id } if *object_id == stack_id)));
    }

    #[test]
    fn finalize_creature_enters_battlefield_with_summoning_sickness() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), creature_chars(2, 2));
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        let entry = s.pop_stack_entry().unwrap();

        s.finalize_resolved_spell(entry);
        let obj = s.objects.objects_in_zone(Zone::Battlefield).next().unwrap();
        assert!(obj.status.summoning_sick);

        let etb = s.event_log.iter().any(|ev|
            matches!(ev, GameEvent::EntersBattlefield { was_cast: true, .. })
        );
        assert!(etb, "expected EntersBattlefield event");
        // The stack-side id becomes LKI; its zone was Stack at that point.
        assert_eq!(s.lki(stack_id).unwrap().zone, Zone::Stack);
    }

    #[test]
    fn finalize_spell_emits_zone_change_stack_to_graveyard() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        let entry = s.pop_stack_entry().unwrap();

        s.finalize_resolved_spell(entry);

        let found = s.event_log.iter().any(|ev| matches!(
            ev,
            GameEvent::ZoneChange {
                object_id, from: Zone::Stack, to: Zone::Graveyard(0),
                cause: MoveCause::SpellResolution, ..
            } if *object_id == stack_id
        ));
        assert!(found, "expected Stack→Graveyard ZoneChange(SpellResolution)");
    }

    // --- Countering --------------------------------------------------------

    #[test]
    fn counter_resolved_spell_moves_to_graveyard_and_emits_event() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        let entry = s.pop_stack_entry().unwrap();

        s.counter_resolved_spell(entry);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|ev|
            matches!(ev, GameEvent::SpellCountered { object_id } if *object_id == stack_id)));
    }

    #[test]
    fn counter_ability_emits_countered_event_no_zone_change() {
        let mut s = GameState::new(2, 0);
        s.push_stack_entry(StackEntry::new_activated_ability(
            42, 10, 0, 1, "T: deal 1".into(),
            TargetSelection::new(), vec![], None));
        let before = s.event_log.len();
        let entry = s.pop_stack_entry().unwrap();
        s.counter_resolved_ability(entry);
        assert_eq!(s.event_log.len(), before + 1);
        assert!(matches!(
            s.event_log.last().unwrap(),
            GameEvent::SpellCountered { object_id: 42 }
        ));
    }

    // --- remove_stack_entry_by_id ------------------------------------------

    #[test]
    fn remove_stack_entry_by_id_removes_middle() {
        let mut s = GameState::new(2, 0);
        for id in [1, 2, 3] {
            s.push_stack_entry(StackEntry::new_spell(
                id, 0, 1, instant_chars(),
                TargetSelection::new(), vec![], None));
        }
        let removed = s.remove_stack_entry_by_id(2).unwrap();
        assert_eq!(removed.id, 2);
        let ids: Vec<_> = s.stack_entries().iter().map(|e| e.id).collect();
        assert_eq!(ids, vec![1, 3]);
    }

    // --- damage_target_is_stack_entry --------------------------------------

    #[test]
    fn damage_target_stack_entry_check() {
        let mut s = GameState::new(2, 0);
        let card = put_object(&mut s, 0, Zone::Hand(0), instant_chars());
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        assert!(damage_target_is_stack_entry(DamageTarget::Object(stack_id), &s));
        assert!(!damage_target_is_stack_entry(DamageTarget::Object(card), &s));
        assert!(!damage_target_is_stack_entry(DamageTarget::Object(999), &s));
        assert!(!damage_target_is_stack_entry(DamageTarget::Player(0), &s));
    }

    // --- Full CR 601 + 608 round-trip --------------------------------------

    #[test]
    fn full_cast_and_resolve_targeted_spell() {
        // Integration sketch: cast Lightning-Bolt-like spell targeting
        // a creature, resolve it normally.
        let mut s = GameState::new(2, 0);
        let creature = put_object(&mut s, 1, Zone::Battlefield, creature_chars(2, 2));
        // Make it opponent-controlled.
        s.objects.get_mut(creature).unwrap().controller = 1;
        let spell = put_object(&mut s, 0, Zone::Hand(0), instant_chars());

        let stack_id = s.announce_spell_on_stack(
            spell, 0, one_creature_target(creature), vec![], None, vec![]);
        s.emit_spell_cast(stack_id);

        let entry = s.pop_stack_entry().unwrap();
        let req = TargetRequirement {
            filter: TargetFilter::Permanent(ObjectFilter::creature()),
            count: TargetCount::Exactly(1),
            controller: None,
        };
        match s.recheck_and_classify_resolution(&entry, std::slice::from_ref(&req)) {
            ResolutionOutcome::Resolve { target_legality } => {
                assert_eq!(target_legality, vec![TargetLegality::Legal]);
            }
            _ => panic!("unexpected counter"),
        }
        s.finalize_resolved_spell(entry);

        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.stack_is_empty());
    }

    #[test]
    fn full_cast_then_countered_by_rules_when_target_vanishes() {
        let mut s = GameState::new(2, 0);
        let creature = put_object(&mut s, 1, Zone::Battlefield, creature_chars(2, 2));
        let spell = put_object(&mut s, 0, Zone::Hand(0), instant_chars());

        let stack_id = s.announce_spell_on_stack(
            spell, 0, one_creature_target(creature), vec![], None, vec![]);
        s.emit_spell_cast(stack_id);

        // In between casting and resolution, target dies.
        s.objects.get_mut(creature).unwrap().zone = Zone::Graveyard(1);

        let entry = s.pop_stack_entry().unwrap();
        let req = target_creature();
        match s.recheck_and_classify_resolution(&entry, std::slice::from_ref(&req)) {
            ResolutionOutcome::CounteredIllegalTargets => {}
            _ => panic!("expected CounteredIllegalTargets"),
        }
        s.counter_resolved_spell(entry);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|ev|
            matches!(ev, GameEvent::SpellCountered { object_id } if *object_id == stack_id)));
    }

    // --- serde roundtrip of a StackEntry (spell) ---------------------------

    #[test]
    fn stack_entry_spell_roundtrip() {
        let entry = StackEntry::new_spell(
            1, 0, 42, instant_chars(),
            TargetSelection { targets: vec![TargetChoice::Player(1)] },
            vec![ModeChoice::new(vec![0, 1])], Some(3),
        );
        let json = serde_json::to_string(&entry).unwrap();
        let back: StackEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 1);
        assert!(back.is_spell());
        assert_eq!(back.x_value, Some(3));
    }
}
