//! Invasion of Regatha // Disciples of the Inferno — `{2}{R}` Battle — Siege with 6 defense
//! counters. ETB: deals 4 damage to another target battle or opponent and 1 damage to up to
//! one target creature.
//!
//! Back face: Disciples of the Inferno — 5/4 Creature — Human Monk with Prowess.
//!
//! # GAPs
//! - "Whenever you cast a noncreature spell, noncreature sources you control deal +2 damage
//!   this turn" is a replacement-modifying static rider, not modeled (no damage-boost
//!   replacement effect). The Prowess keyword itself is on the back face.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Regatha");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Disciples of the Inferno
    let back_name = reg.interner_mut().intern("Disciples of the Inferno");
    let human_sub = reg.interner_mut().intern("Human");
    let monk_sub = reg.interner_mut().intern("Monk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(monk_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Prowess],
            // GAP: "noncreature source deals +2 damage this turn" replacement-modifying
            //      rider not modeled (no damage-boost replacement effect primitive).
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    // 4 damage to another target battle or opponent (any target)
                    TargetRequirement {
                        filter: TargetFilter::AnyTarget,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    // 1 damage to up to one target creature
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
    )
}

fn etb_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();

    // First target (any target): deal 4 damage
    if let Some(target) = trig.targets.targets.first() {
        let dt = match target {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
            _ => return effects,
        };
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: dt,
            amount: 4,
        });
    }

    // Second target (up to one creature): deal 1 damage
    if let Some(target) = trig.targets.targets.get(1) {
        let TargetChoice::Object(id) = target else { return effects; };
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(*id),
            amount: 1,
        });
    }

    effects
}
