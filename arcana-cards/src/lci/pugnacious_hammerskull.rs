//! Pugnacious Hammerskull — `{2}{G}` 6/6 green Creature — Dinosaur.
//! "Whenever this creature attacks while you don't control another
//! Dinosaur, put a stun counter on it."
//! GAP: intervening-if condition ("while you don't control another
//! Dinosaur") is not modeled — no intervening_if API. The stun counter
//! (`CounterKind::Stun`) may not exist; using the SelfAttacks trigger
//! and checking at resolution via script::count_matching, returning
//! Vec::new() if another Dinosaur is present.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pugnacious Hammerskull");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: intervening-if — "while you don't control another
                // Dinosaur" is not modeled via intervening_if API; checking
                // at resolution instead.
                intervening_if: None,
                effect: maybe_stun_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_stun_self(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Count other Dinosaurs you control (excluding this creature).
    // The filter counts ALL Dinosaurs; since this creature is attacking
    // it's still on battlefield, so subtract 1 (self).
    let dinosaur_filter = script::subtype_filter(reg, "Dinosaur")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &dinosaur_filter, trig.controller);
    // If there's more than 1 (i.e. another Dinosaur besides self), don't trigger.
    if n > 1 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Stun,
        count: 1,
    }]
}
