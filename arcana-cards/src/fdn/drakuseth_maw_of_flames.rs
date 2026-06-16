//! Drakuseth, Maw of Flames — `{4}{R}{R}{R}` 7/7 Legendary Dragon, Flying.
//! "Whenever Drakuseth attacks, it deals 4 damage to any target and 3
//! damage to each of up to two other targets."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drakuseth, Maw of Flames");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: drakuseth_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                // The "4 damage to any target".
                TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                // The "3 damage to each of up to two other targets".
                TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::UpTo(2),
                    controller: None,
                },
            ],
        }),
    )
}

fn to_dt(choice: &TargetChoice) -> Option<DamageTarget> {
    match choice {
        TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
        TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => Some(DamageTarget::Object(*id)),
            ObjectOrPlayer::Player(p) => Some(DamageTarget::Player(*p)),
        },
    }
}

fn drakuseth_damage(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = Vec::new();
    let mut targets = trig.targets.targets.iter();
    // First chosen target: 4 damage.
    if let Some(first) = targets.next() {
        if let Some(dt) = to_dt(first) {
            out.push(Effect::DealDamage {
                source: trig.source,
                target: dt,
                amount: 4,
            });
        }
    }
    // Remaining (up to two) targets: 3 damage each.
    for choice in targets {
        if let Some(dt) = to_dt(choice) {
            out.push(Effect::DealDamage {
                source: trig.source,
                target: dt,
                amount: 3,
            });
        }
    }
    out
}
