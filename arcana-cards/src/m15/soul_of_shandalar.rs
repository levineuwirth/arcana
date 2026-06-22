//! Soul of Shandalar — `{4}{R}{R}` 6/6 Avatar with First strike.
//!
//! Oracle:
//! * First strike — base keyword.
//! * `{3}{R}{R}: This creature deals 3 damage to target player or
//!   planeswalker and 3 damage to up to one target creature that
//!   player or that planeswalker's controller controls.`
//! * `{3}{R}{R}, Exile this card from your graveyard: It deals 3
//!   damage to target player or planeswalker and 3 damage to up to one
//!   target creature that player or that planeswalker's controller
//!   controls.` — the same payload, activated from the graveyard with
//!   an exile-self cost.
//!
//! "target player or planeswalker" is approximated as a player target
//! (planeswalker as a damage target isn't separately expressible here).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul of Shandalar");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{R}: This creature deals 3 damage to target player or planeswalker and 3 damage to up to one target creature that player or that planeswalker's controller controls.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature()),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bolt_both,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{R}, Exile this card from your graveyard: It deals 3 damage to target player or planeswalker and 3 damage to up to one target creature that player or that planeswalker's controller controls.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{R}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature()),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: bolt_both,
            }),
    )
}

fn bolt_both(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in ctx.targets.targets.iter() {
        match t {
            TargetChoice::Player(p) => effects.push(Effect::DealDamage {
                source: ctx.source,
                target: DamageTarget::Player(*p),
                amount: 3,
            }),
            TargetChoice::Object(id) => effects.push(Effect::DealDamage {
                source: ctx.source,
                target: DamageTarget::Object(*id),
                amount: 3,
            }),
            _ => {}
        }
    }
    effects
}
