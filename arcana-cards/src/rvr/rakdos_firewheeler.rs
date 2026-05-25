//! Rakdos Firewheeler — `{B}{B}{R}{R}` 4/3 black-red Human Rogue.
//! "When this creature enters, it deals 2 damage to target opponent and 2 damage to up to
//! one target creature or planeswalker."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos Firewheeler");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: deal_damage_two_targets,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature()),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn deal_damage_two_targets(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // First target: opponent player
    if let Some(first) = trig.targets.targets.first() {
        if let TargetChoice::Player(p) = first {
            effects.push(Effect::DealDamage {
                target: DamageTarget::Player(*p),
                amount: 2,
                source: trig.source,
            });
        }
    }
    // Second target: up to one creature/planeswalker
    if let Some(second) = trig.targets.targets.get(1) {
        if let TargetChoice::Object(id) = second {
            effects.push(Effect::DealDamage {
                target: DamageTarget::Object(*id),
                amount: 2,
                source: trig.source,
            });
        }
    }
    effects
}
