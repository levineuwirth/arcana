//! Skullmane Baku — `{3}{B}{B}` 2/1 black Spirit.
//!
//! * "Whenever you cast a Spirit or Arcane spell, you may put a ki
//!   counter on this creature." — a `SpellCast` trigger filtered to the
//!   Spirit / Arcane subtypes; the "may" is a resolution-time choice, so
//!   the effect adds a `ki` counter (a named counter).
//! * "{1}, {T}, Remove X ki counters from this creature: Target creature
//!   gets -X/-X until end of turn." — the cost removes a VARIABLE number
//!   of `ki` counters and the magnitude X feeds the -X/-X. The demonstrated
//!   `ActivationCost` only supports a FIXED `remove_self_counter` count and
//!   the effect has no way to read the count of counters removed as the
//!   activation's X, so this variable-X cost/payload pair is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skullmane Baku");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    // Subtype-OR filter for "a Spirit or Arcane spell".
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit, arcane]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
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
                text: "{1}, {T}, Remove X ki counters from Skullmane Baku: Target creature gets -X/-X until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: remove_x_ki_minus_x,
            }),
    )
}

/// "you may put a ki counter on this creature" — add a named `ki` counter
/// to the source. (The "may" is a resolution choice; we apply it.)
fn add_ki_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ki = match reg.interner().lookup("ki") {
        Some(k) => k,
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(ki),
        count: 1,
    }]
}

/// "Remove X ki counters: target creature gets -X/-X" — GAP: the cost removes
/// a VARIABLE number of ki counters (X) and the effect's magnitude is that
/// same X. The demonstrated ActivationCost only takes a FIXED counter-removal
/// count and the effect fn has no accessor for the X paid as the counter cost,
/// so the variable-X cost/payload is unexpressible.
fn remove_x_ki_minus_x(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: variable X = number of ki counters removed; no fixed remove_self_counter / X accessor.
    let _ = Duration::EndOfTurn;
    Vec::new()
}
