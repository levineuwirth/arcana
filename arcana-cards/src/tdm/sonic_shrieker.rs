//! Sonic Shrieker — `{2}{R}{W}{B}` 4/4 Mardu Dragon with Flying.
//!
//! Flying
//! When this creature enters, it deals 2 damage to any target and you gain 2
//! life. If a player is dealt damage this way, they discard a card.
//!
//! Flying is a base keyword. The ETB targets any target: it deals 2 damage, the
//! controller gains 2 life, and — when the damaged target is a player — that
//! player discards a card.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sonic Shrieker");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_blast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn etb_blast(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let (dt, player_hit) = match target {
        TargetChoice::Object(id) => (DamageTarget::Object(*id), None),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), Some(*p)),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => (DamageTarget::Object(*id), None),
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), Some(*p)),
        },
        _ => return Vec::new(),
    };
    let mut effects = vec![
        Effect::DealDamage { source: trig.source, target: dt, amount: 2 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ];
    if let Some(p) = player_hit {
        effects.push(Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects
}
