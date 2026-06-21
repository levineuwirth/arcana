//! Quillmane Baku — `{4}{U}` 3/3 Spirit.
//! Whenever you cast a Spirit or Arcane spell, you may put a ki counter
//! on this creature. (SpellCast filtered to Spirit/Arcane subtypes; the
//! "may" is a resolution choice we apply, adding a named `ki` counter.)
//! {1}, {T}, Remove X ki counters from this creature: Return target
//! creature with mana value X or less to its owner's hand. (GAP — the
//! ActivationCost only takes a FIXED remove_self_counter count, and the
//! effect has no accessor for the X ki removed, so the variable-X cost
//! and the X-bounded target filter are unexpressible.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quillmane Baku");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let _ki = reg.interner_mut().intern("ki");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    // Subtype-OR filter for "a Spirit or Arcane spell".
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit, arcane]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
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
                text: "{1}, {T}, Remove X ki counters from Quillmane Baku: Return \
                       target creature with mana value X or less to its owner's hand."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: remove_x_ki_bounce,
            }),
    )
}

/// "you may put a ki counter on this creature" — add a named `ki` counter
/// to the source. (The "may" is a resolution choice; we apply it.)
fn add_ki_counter(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
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

/// GAP: the cost removes a VARIABLE number of ki counters (X) and the
/// target filter is "mana value X or less". The demonstrated ActivationCost
/// only takes a FIXED remove_self_counter count, and the effect fn has no
/// accessor for the X paid, so the variable-X cost and X-bounded target are
/// unexpressible.
fn remove_x_ki_bounce(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
