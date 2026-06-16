//! Domri Rade — `{1}{R}{G}` Legendary Planeswalker — Domri, starting loyalty 3.
//! +1: Look at the top card of your library; if a creature, you may reveal it and put it into
//!     your hand. — no look-at-top primitive → GAP'd (shell with correct +1 cost).
//! −2: Target creature you control fights another target creature. (two targets, Fight)
//! −7: Emblem — STATIC "Creatures you control have double strike, trample, hexproof, and haste."
//!     IMPLEMENTED via four keyword_anthem statics.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri Rade");
    let sub = reg.interner_mut().intern("Domri");
    let _emblem = reg.interner_mut().intern("Domri Rade emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top card of your library. If it's a creature card, \
                       you may reveal it and put it into your hand."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target creature you control fights another target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .controlled_by(ControllerConstraint::You)
                                .with_types(TypeLine::CREATURE.into()),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_creature(),
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_fight,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Creatures you control have double strike, \
                       trample, hexproof, and haste.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+1` — GAP: look at top card / may-reveal has no demonstrated primitive.
fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}

/// `−2: Target creature you control fights another target creature.`
fn minus_two_fight(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let a = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let b = match ctx.targets.targets.get(1) {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::Fight { a, b }]
}

/// `−7: emblem — creatures you control have double strike, trample, hexproof, and haste.`
fn minus_seven(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Domri Rade emblem")
        .expect("emblem name interned");
    let null = arcana_core::objects::NULL_OBJECT_ID;
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![
                ContinuousEffect::keyword_anthem(
                    null,
                    ctx.controller,
                    KeywordAbility::DoubleStrike,
                    Duration::Permanent,
                ),
                ContinuousEffect::keyword_anthem(
                    null,
                    ctx.controller,
                    KeywordAbility::Trample,
                    Duration::Permanent,
                ),
                ContinuousEffect::keyword_anthem(
                    null,
                    ctx.controller,
                    KeywordAbility::Hexproof,
                    Duration::Permanent,
                ),
                ContinuousEffect::keyword_anthem(
                    null,
                    ctx.controller,
                    KeywordAbility::Haste,
                    Duration::Permanent,
                ),
            ],
            abilities: vec![],
        },
    }]
}
