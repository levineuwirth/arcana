//! Burning Sun's Avatar — `{3}{R}{R}{R}` 6/6 red Dinosaur Avatar.
//! "When this creature enters, it deals 3 damage to target opponent or planeswalker
//! and 3 damage to up to one target creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burning Sun's Avatar");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: deal_damage_double,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn deal_damage_double(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // First target: opponent or planeswalker
    if let Some(target) = trig.targets.targets.first() {
        let dt = match target {
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => Some(DamageTarget::Object(*id)),
                ObjectOrPlayer::Player(p) => Some(DamageTarget::Player(*p)),
            },
            _ => None,
        };
        if let Some(dt) = dt {
            effects.push(Effect::DealDamage { target: dt, amount: 3, source: trig.source });
        }
    }
    // Second target: up to one creature
    if let Some(target) = trig.targets.targets.get(1) {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DealDamage {
                target: DamageTarget::Object(*id),
                amount: 3,
                source: trig.source,
            });
        }
    }
    effects
}
