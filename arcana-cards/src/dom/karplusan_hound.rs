//! Karplusan Hound — `{3}{R}` 3/3 red Dog.
//! "Whenever this creature attacks, if you control a Chandra
//! planeswalker, this creature deals 2 damage to any target."
//! Intervening-if "if you control a Chandra planeswalker" modeled via
//! `conditions::you_control_subtype` ("Chandra" is the planeswalker subtype).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karplusan Hound");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(iif_control_chandra),
                effect: attacks_deal_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn iif_control_chandra(state: &GameState, _source: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype(state, reg, you, "Chandra")
}

fn attacks_deal_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    match target {
        TargetChoice::Object(id) => vec![Effect::DealDamage {
            target: DamageTarget::Object(*id),
            amount: 2,
            source: trig.source,
        }],
        TargetChoice::Player(p) => vec![Effect::DealDamage {
            target: DamageTarget::Player(*p),
            amount: 2,
            source: trig.source,
        }],
        TargetChoice::ObjectOrPlayer(op) => match op {
            ObjectOrPlayer::Object(id) => vec![Effect::DealDamage {
                target: DamageTarget::Object(*id),
                amount: 2,
                source: trig.source,
            }],
            ObjectOrPlayer::Player(p) => vec![Effect::DealDamage {
                target: DamageTarget::Player(*p),
                amount: 2,
                source: trig.source,
            }],
        },
        _ => Vec::new(),
    }
}
