//! Samut, the Tested — `{2}{R}{G}` Legendary Planeswalker — Samut, starting loyalty 5.
//!
//! +1: Up to one target creature gains double strike until end of turn.
//! −2: Samut deals 2 damage divided as you choose among one or two targets.
//!   Modeled with `Effect::DealDamageDivided` over up-to-two any-targets.
//! −7: Search your library for up to two creature and/or planeswalker cards, put
//!   them onto the battlefield, then shuffle. Modeled as two `TutorToBattlefield`
//!   over a creature-or-planeswalker filter (each tutor includes the shuffle).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Samut, the Tested");
    let samut = reg.interner_mut().intern("Samut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(samut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gains double strike until end \
                       of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_double_strike,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Samut deals 2 damage divided as you choose among one or \
                       two targets.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_divided,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Search your library for up to two creature and/or \
                       planeswalker cards, put them onto the battlefield, then \
                       shuffle.".into(),
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
                effect: minus_seven_tutor,
            }),
    )
}

fn plus_one_double_strike(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::DoubleStrike,
        duration: Duration::EndOfTurn,
    }]
}

fn minus_two_divided(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut targets = Vec::new();
    for t in &ctx.targets.targets {
        match t {
            TargetChoice::Player(p) => targets.push(DamageTarget::Player(*p)),
            TargetChoice::Object(id) => targets.push(DamageTarget::Object(*id)),
            _ => {}
        }
    }
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::DealDamageDivided {
        source: ctx.source,
        targets,
        total: 2,
    }]
}

fn minus_seven_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Creature OR planeswalker filter (types_any disjunction).
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER));
    vec![
        Effect::TutorToBattlefield { player: ctx.controller, filter: filter.clone(), tapped: false },
        Effect::TutorToBattlefield { player: ctx.controller, filter, tapped: false },
    ]
}
