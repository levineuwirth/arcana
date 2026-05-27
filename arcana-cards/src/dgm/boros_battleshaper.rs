//! Boros Battleshaper — `{5}{R}{W}` 5/5 red/white Minotaur Soldier.
//! "At the beginning of each combat, up to one target creature attacks or blocks this combat
//! if able and up to one target creature can't attack or block this combat."
//! GAP: "attacks or blocks if able" forced-attack/block constraint not modeled;
//! two-target modal pattern with one target getting Goad and one getting ForbidAttacking
//! partially modeled — using Goad for the forced-attack target and ForbidAttacking for the second.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boros Battleshaper");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: combat_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn combat_effect(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "attacks or blocks if able" forced-attack/block not modeled; using Goad as partial stand-in
    let mut effects = Vec::new();
    if let Some(first) = trig.targets.targets.first() {
        if let TargetChoice::Object(id) = first {
            effects.push(Effect::Goad { target: *id, goader: trig.controller, duration: Duration::EndOfTurn });
        }
    }
    if let Some(second) = trig.targets.targets.get(1) {
        if let TargetChoice::Object(id) = second {
            effects.push(Effect::ForbidAttacking { target: *id, duration: Duration::EndOfTurn });
        }
    }
    effects
}
