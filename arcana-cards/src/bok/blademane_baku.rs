//! Blademane Baku — `{1}{R}` 1/1 Spirit.
//! "Whenever you cast a Spirit or Arcane spell, you may put a ki counter
//! on this creature."
//! "{1}, Remove X ki counters from this creature: For each counter
//! removed, this creature gets +2/+0 until end of turn."
//!
//! The cast trigger filters on the Spirit/Arcane subtypes and adds a ki
//! counter (modeled non-optionally — the "may" is a minor fidelity gap).
//! ki has no dedicated CounterKind, so `CounterKind::Named("ki")`.
//! The activated ability is GAP'd: there is no "remove X counters" cost
//! field (`remove_loyalty_x` is loyalty-only; `remove_self_counter`
//! removes a fixed N, not a chosen X).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blademane Baku");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let spirit_sub = reg.interner_mut().intern("Spirit");
    let arcane_sub = reg.interner_mut().intern("Arcane");
    let _ki = reg.interner_mut().intern("ki");
    let spell_filter = ObjectFilter::new()
        .with_subtypes_any(vec![spirit_sub, arcane_sub])
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
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
        }),
        // GAP: "{1}, Remove X ki counters: +2/+0 per counter" — no
        // "remove X counters" cost field; activated ability omitted.
    )
}

fn add_ki_counter(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let ki = match reg.interner().lookup("ki") {
        Some(s) => s,
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(ki),
        count: 1,
    }]
}
