//! Ebon Praetor — `{4}{B}{B}` 5/5 Avatar Praetor with First strike and Trample.
//! "At the beginning of your upkeep, put a -2/-2 counter on this creature."
//! "Sacrifice a creature: Remove a -2/-2 counter from this creature. If the sacrificed
//! creature was a Thrull, put a +1/+0 counter on this creature. Activate only during your
//! upkeep and only once each turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ebon Praetor");
    let avatar = reg.interner_mut().intern("Avatar");
    let praetor = reg.interner_mut().intern("Praetor");
    let _minus = reg.interner_mut().intern("-2/-2");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(praetor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_minus,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a creature: Remove a -2/-2 counter from this creature. If the sacrificed creature was a Thrull, put a +1/+0 counter on this creature. Activate only during your upkeep and only once each turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: remove_minus,
            }),
    )
}

fn upkeep_minus(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let kind = reg
        .interner()
        .lookup("-2/-2")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::MinusOneMinusOne);
    vec![Effect::AddCounters { target: trig.source, kind, count: 1 }]
}

fn remove_minus(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // "Remove a -2/-2 counter from this creature." GAP: the "only during your upkeep"
    // timing window and the "if the sacrificed creature was a Thrull, put a +1/+0 counter"
    // rider (the sacrificed permanent's identity isn't readable here) are not expressed.
    let kind = reg
        .interner()
        .lookup("-2/-2")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::MinusOneMinusOne);
    vec![Effect::RemoveCounters { target: ctx.source, kind, count: 1 }]
}
