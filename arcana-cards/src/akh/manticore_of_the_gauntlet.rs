//! Manticore of the Gauntlet — `{4}{R}` 5/4 red Manticore.
//! "When this creature enters, put a -1/-1 counter on target creature you control. This creature deals 3 damage to target opponent or planeswalker."
//! GAP: "target opponent or planeswalker" — planeswalker targeting not in catalog; using target player as proxy.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Manticore of the Gauntlet");
    let manticore = reg.interner_mut().intern("Manticore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(manticore);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_debuff_and_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::You),
                    },
                    // GAP: "opponent or planeswalker" — planeswalker not in catalog; using Player
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                ],
            }),
    )
}

fn etb_debuff_and_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for (i, target) in trig.targets.targets.iter().enumerate() {
        match (i, target) {
            (0, TargetChoice::Object(id)) => {
                effects.push(Effect::AddCounters {
                    target: *id,
                    kind: CounterKind::MinusOneMinusOne,
                    count: 1,
                });
            }
            (1, TargetChoice::Player(p)) => {
                effects.push(Effect::DealDamage {
                    target: DamageTarget::Player(*p),
                    amount: 3,
                    source: trig.source,
                });
            }
            _ => {}
        }
    }
    effects
}
