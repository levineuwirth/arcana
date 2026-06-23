//! Festercreep — `{1}{B}` 0/0 Elemental.
//!
//! Oracle:
//! * This creature enters with a +1/+1 counter on it.
//! * {1}{B}, Remove a +1/+1 counter from this creature: All other creatures
//!   get -1/-1 until end of turn.
//!
//! "Enters with a +1/+1 counter" is modeled as an ETB trigger that adds one
//! +1/+1 counter to itself. The activated ability pays {1}{B} plus removing
//! a +1/+1 counter (via `remove_self_counter`), then gives every OTHER
//! creature -1/-1 until end of turn (one Pump per matching id, excluding
//! this creature, wrapped in a Sequence).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Festercreep");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Enters with a +1/+1 counter on it."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{1}{B}, Remove a +1/+1 counter: all other creatures -1/-1 EOT."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, Remove a +1/+1 counter from this creature: All other creatures get -1/-1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_all_others,
            }),
    )
}

fn etb_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.entering_object().unwrap_or(trig.source),
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn minus_all_others(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let pumps: Vec<Effect> = ids
        .into_iter()
        .filter(|&id| id != ctx.source)
        .map(|id| Effect::Pump {
            target: id,
            power: -1,
            toughness: -1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    vec![Effect::Sequence(pumps)]
}
