//! Ral, Izzet Viceroy — `{3}{U}{R}` Legendary Planeswalker — Ral,
//! starting loyalty 5.
//!
//! +1: Look at the top two cards of your library. Put one of them into your
//!     hand and the other into your graveyard.
//! −3: Ral deals damage to target creature equal to the total number of instant
//!     and sorcery cards you own in exile and in your graveyard.
//! −8: You get an emblem with "Whenever you cast an instant or sorcery spell,
//!     this emblem deals 4 damage to any target and you draw two cards."
//!
//! # Scope
//! - The `+1` is a look-at-top-2 dig: take one to hand, the rest (the other) to
//!   the graveyard — DigTopN { count: 2, filter: None, rest: Graveyard }.
//! - The `−3` deals damage equal to a count of instant/sorcery cards across BOTH
//!   exile and graveyard "you own". The demonstrated script surface has a
//!   graveyard counter but no exile-zone / cross-zone "you own" counter, and
//!   DealDamage takes a fixed amount; the cross-zone dynamic amount is GAP'd.
//!   Correct `−3` cost + creature target shell retained.
//! - The `−8` is a TRIGGERED EMBLEM (now supported): whenever you cast an
//!   instant or sorcery spell, the emblem deals 4 to any target and you draw 2.

use arcana_core::effects::{DigRest, Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Izzet Viceroy");
    let ral = reg.interner_mut().intern("Ral");
    let _emblem = reg.interner_mut().intern("Ral, Izzet Viceroy emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ral);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top two cards of your library. Put one of \
                       them into your hand and the other into your graveyard."
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
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Ral deals damage to target creature equal to the total \
                       number of instant and sorcery cards you own in exile and \
                       in your graveyard."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever you cast an instant \
                       or sorcery spell, this emblem deals 4 damage to any target \
                       and you draw two cards.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+1: Look at the top two; one to hand, the other to graveyard.`
fn plus_one_dig(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 2,
        filter: None,
        rest: DigRest::Graveyard,
    }]
}

/// `−3: Ral deals damage to target creature = instant/sorcery in exile + gy.`
fn minus_three_damage(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: damage equal to a count of instant/sorcery cards across BOTH exile
    // and graveyard "you own". No exile-zone / cross-zone "you own" counter in
    // the demonstrated script surface, and DealDamage takes a fixed amount.
    // Correct `−3` cost + creature target shell retained.
    Vec::new()
}

/// `−8: Triggered emblem — cast instant/sorcery → deal 4 to any target, draw 2.`
fn minus_eight_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Ral, Izzet Viceroy emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_cast_trigger,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }],
        },
    }]
}

/// Emblem trigger: deal 4 to any target, draw two cards.
fn emblem_cast_trigger(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let dt = match trig.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id))) => DamageTarget::Object(*id),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p))) => DamageTarget::Player(*p),
        _ => return vec![Effect::DrawCards { player: trig.controller, count: 2 }],
    };
    vec![
        Effect::DealDamage {
            source: NULL_OBJECT_ID,
            target: dt,
            amount: 4,
        },
        Effect::DrawCards { player: trig.controller, count: 2 },
    ]
}
