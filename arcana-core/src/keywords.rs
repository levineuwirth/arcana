//! Keyword-ability expansions: builders that convert a
//! [`crate::effects::KeywordAbility`] into the concrete
//! [`TriggeredAbilityDef`] / replacement / static-ability objects the
//! engine actually dispatches on.
//!
//! Each builder returns a fully-formed def that a card definition can
//! push onto its `triggered_abilities` / `replacement_effects` list at
//! registration time. This keeps the `KeywordAbility` enum as a
//! marker / cost-carrier and moves the wiring into one canonical
//! place so keyword behavior doesn't drift between cards.

use crate::effects::Effect;
use crate::events::GameEvent;
use crate::triggers::{TriggeredAbilityDef, TriggerCondition, TriggerFrequency};
use crate::types::TriggerId;
use crate::zones::Zone;

// =============================================================================
// Storm — CR 702.40
// =============================================================================

/// Build a storm trigger for a card. Fires when the card itself is
/// cast; resolution produces N [`Effect::CopySpell`] effects where N
/// is the cast entry's
/// [`crate::stack::StackEntry::storm_count_at_cast`] — i.e. the
/// number of other spells cast before this one this turn.
///
/// Each [`Effect::CopySpell`] independently pushes a
/// [`crate::actions::ChoiceKind::ChooseTargets`] if the copied spell
/// had targets (CR 706.10). Copies go on the stack above the original
/// and resolve LIFO, so the original resolves last.
///
/// `trigger_id` lets the card namespace its triggers. Pass any
/// monotonic number; the engine only uses it for frequency accounting.
pub fn storm_trigger_def(trigger_id: TriggerId) -> TriggeredAbilityDef {
    TriggeredAbilityDef {
        id: trigger_id,
        // "When ~ is cast" — match SpellCast where the cast id equals
        // this trigger's source (the storm card itself on the stack).
        trigger_condition: TriggerCondition::Custom(storm_fires_on_self_cast),
        intervening_if: None,
        effect: storm_effect,
        // The card is on the stack when it's being cast.
        trigger_zones: vec![Zone::Stack],
        frequency: TriggerFrequency::EachTime,
    }
}

fn storm_fires_on_self_cast(
    event: &GameEvent,
    _state: &crate::state::GameState,
    source: crate::objects::ObjectId,
) -> bool {
    matches!(event, GameEvent::SpellCast { object_id, .. } if *object_id == source)
}

fn storm_effect(
    state: &crate::state::GameState,
    pt: &crate::triggers::PendingTrigger,
    _: &crate::registry::CardRegistry,
) -> Vec<Effect> {
    // The cast id is in the triggering event; the stack entry snapshots
    // storm_count_at_cast at announce time.
    let cast_id = match pt.trigger_event {
        GameEvent::SpellCast { object_id, .. } => object_id,
        _ => return Vec::new(),
    };
    let n = state.stack.iter()
        .find(|e| e.id == cast_id)
        .map(|e| e.storm_count_at_cast)
        .unwrap_or(0);
    (0..n).map(|_| Effect::CopySpell { target: cast_id }).collect()
}

// =============================================================================
// Cascade — CR 702.85
// =============================================================================

/// Build a cascade trigger for a card. Fires when the card is cast;
/// resolution exiles cards off the top of the caster's library until
/// a nonland with lesser mana value appears, offers a may-cast choice,
/// and returns the remaining exiles to the bottom of the library in
/// seeded-random order.
pub fn cascade_trigger_def(trigger_id: TriggerId) -> TriggeredAbilityDef {
    TriggeredAbilityDef {
        id: trigger_id,
        trigger_condition: TriggerCondition::Custom(cascade_fires_on_self_cast),
        intervening_if: None,
        effect: cascade_effect,
        trigger_zones: vec![Zone::Stack],
        frequency: TriggerFrequency::EachTime,
    }
}

fn cascade_fires_on_self_cast(
    event: &GameEvent,
    _state: &crate::state::GameState,
    source: crate::objects::ObjectId,
) -> bool {
    matches!(event, GameEvent::SpellCast { object_id, .. } if *object_id == source)
}

fn cascade_effect(
    _state: &crate::state::GameState,
    pt: &crate::triggers::PendingTrigger,
    _: &crate::registry::CardRegistry,
) -> Vec<Effect> {
    let cast_id = match pt.trigger_event {
        GameEvent::SpellCast { object_id, .. } => object_id,
        _ => return Vec::new(),
    };
    vec![Effect::Cascade { source: cast_id, controller: pt.controller }]
}
