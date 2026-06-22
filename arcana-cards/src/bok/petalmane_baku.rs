//! Petalmane Baku — `{1}{G}` 1/2 green Spirit.
//!
//! * "Whenever you cast a Spirit or Arcane spell, you may put a ki counter on
//!   this creature." — a `SpellCast` trigger filtered to the Spirit / Arcane
//!   subtypes; the "may" is a resolution-time choice, so the effect adds a
//!   named `ki` counter to the source.
//! * "{1}, Remove X ki counters from this creature: Add X mana of any one
//!   color." — GAP: the cost removes a VARIABLE number of ki counters (X) and
//!   the payload mints X mana of a chosen color. The demonstrated
//!   `ActivationCost` only takes a FIXED counter-removal count, the effect fn
//!   has no accessor for the X paid, and `AddMana` mints only a specific
//!   `ManaColor` pip (no any-color / player-chosen-color primitive). Both the
//!   variable-X cost and the any-color payload are unexpressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Petalmane Baku");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let _ki = reg.interner_mut().intern("ki");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    // Subtype-OR filter for "a Spirit or Arcane spell".
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit, arcane]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_ki_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove X ki counters from Petalmane Baku: Add X mana of any one color.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_x_mana_any_color,
            }),
    )
}

/// "you may put a ki counter on this creature" — add a named `ki` counter to
/// the source. (The "may" is a resolution choice; we apply it.)
fn add_ki_counter(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(ki) = reg.interner().lookup("ki").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: ki,
        count: 1,
    }]
}

/// "Remove X ki counters: Add X mana of any one color" — GAP: variable-X
/// counter cost and an any-color mana payload, neither expressible (no X
/// accessor on the cost, no any-color AddMana primitive).
fn add_x_mana_any_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: variable X = ki counters removed; any-color mana payload.
    Vec::new()
}
