//! Phlage, Titan of Fire's Fury — `{1}{R}{W}` 6/6 Legendary Elder Giant.
//! "When Phlage enters, sacrifice it unless it escaped.
//!  Whenever Phlage enters or attacks, it deals 3 damage to any target and you
//!  gain 3 life.
//!  Escape—{R}{R}{W}{W}, Exile five other cards from your graveyard."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phlage, Titan of Fire's Fury");
    let giant = reg.interner_mut().intern("Giant");
    let elder = reg.interner_mut().intern("Elder");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Escape is not in the usable KeywordAbility surface; the escape
        // cast cost (Escape—{R}{R}{W}{W}, exile five cards) is omitted.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "When Phlage enters, sacrifice it unless it escaped" — the
            // "unless it escaped" gate (whether the card was cast for its escape
            // cost) is not observable, and there is no blessed sacrifice-self
            // effect, so this trigger is omitted entirely.
            // "Whenever Phlage enters …, deal 3 damage to any target and gain 3 life."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: bolt_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            // "Whenever Phlage … attacks, deal 3 damage to any target and gain 3 life."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: bolt_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn bolt_and_gain(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![
        Effect::DealDamage {
            source: trig.source,
            target: dt,
            amount: 3,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 3,
        },
    ]
}
