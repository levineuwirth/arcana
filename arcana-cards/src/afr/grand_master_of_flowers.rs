//! Grand Master of Flowers — `{2}{W}{W}` Legendary Planeswalker — Bahamut,
//! starting loyalty 5 — colors W.
//!
//! Oracle text:
//! * Static: As long as Grand Master of Flowers has 7 or more loyalty
//!   counters on him, he's a 7/7 Dragon God creature with flying and
//!   indestructible. — a continuous self-modifying static ability, not a
//!   loyalty ability. GAP: loyalty-gated becomes-a-creature static is not
//!   expressible here and is not represented as an ability.
//! * `+1`: Up to one target creature without first strike, double strike, or
//!   vigilance can't attack or block until your next turn. —
//!   `Effect::ForbidAttacking` + `Effect::ForbidBlocking`. Approximations
//!   noted: the duration is `EndOfTurn` (until-your-next-turn not exact), and
//!   the multi-keyword exclusion filter only excludes first strike
//!   (`without_keyword` takes a single keyword).
//! * `+1`: Search your library and/or graveyard for a card named Monk of the
//!   Open Hand, reveal it, and put it into your hand. — named-card search
//!   across library and graveyard. GAP: named-card tutor + graveyard search
//!   not expressible.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grand Master of Flowers");
    let bahamut = reg.interner_mut().intern("Bahamut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bahamut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature without first strike, \
                       double strike, or vigilance can't attack or block \
                       until your next turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .without_keyword(KeywordAbility::FirstStrike),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_lockdown,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Search your library and/or graveyard for a card \
                       named Monk of the Open Hand, reveal it, and put it into \
                       your hand. If you searched your library this way, \
                       shuffle.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_tutor_named,
            }),
    )
}

/// `+1`: target creature can't attack or block (duration/filter approximated).
fn plus_one_lockdown(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Approximations: duration EndOfTurn (until-your-next-turn not exact);
    // keyword-exclusion filter only excludes first strike.
    let id = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![
        Effect::ForbidAttacking { target: id, duration: Duration::EndOfTurn },
        Effect::ForbidBlocking { target: id, duration: Duration::EndOfTurn },
    ]
}

/// `+1`: named-card tutor across library and graveyard.
fn plus_one_tutor_named(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: named-card tutor (TutorToHand filters by subtype/type, not by name)
    // + graveyard search not expressible from the demonstrated Effect surface.
    Vec::new()
}
