//! Vraska's Conquistador — `{1}{B}` 2/1 black Creature — Vampire Soldier.
//! "Whenever this creature attacks or blocks, if you control a Vraska
//! planeswalker, target opponent loses 2 life and you gain 2 life."
//! Intervening-if "if you control a Vraska planeswalker" modeled via
//! `conditions::you_control_subtype` on both triggers (attacks / blocks).
//! Target opponent is read from target_requirements as a Player target.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska's Conquistador");
    let vampire = reg.interner_mut().intern("Vampire");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    let target_reqs = vec![TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    }];
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(iif_control_vraska),
                effect: drain_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: target_reqs.clone(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: Some(iif_control_vraska),
                effect: drain_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: target_reqs,
            }),
    )
}

fn iif_control_vraska(state: &GameState, _source: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype(state, reg, you, "Vraska")
}

fn drain_opponent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![
        Effect::LoseLife { player: *p, amount: 2 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ]
}
