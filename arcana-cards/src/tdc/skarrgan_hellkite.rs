//! Skarrgan Hellkite — `{3}{R}{R}` 4/4 red Dragon.
//!
//! Oracle:
//! * Riot (enters with a +1/+1 counter or haste).
//! * Flying.
//! * {3}{R}: This creature deals 2 damage divided as you choose among one or
//!   two targets. Activate only if this creature has a +1/+1 counter on it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skarrgan Hellkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Riot, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{R}: This creature deals 2 damage divided as you choose among one or two targets. Activate only if this creature has a +1/+1 counter on it.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{R}").expect("valid cost"),
                // "Activate only if this creature has a +1/+1 counter on it" —
                // pure precondition (counter is NOT removed).
                min_self_counters: Some((CounterKind::PlusOnePlusOne, 1)),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::AnyTarget,
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: deal_two_divided,
        }),
    )
}

fn deal_two_divided(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets: Vec<DamageTarget> = ctx
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            TargetChoice::ObjectOrPlayer(o) => match o {
                arcana_core::targets::ObjectOrPlayer::Object(id) => {
                    Some(DamageTarget::Object(*id))
                }
                arcana_core::targets::ObjectOrPlayer::Player(p) => {
                    Some(DamageTarget::Player(*p))
                }
            },
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::DealDamageDivided {
        source: ctx.source,
        targets,
        total: 2,
    }]
}
