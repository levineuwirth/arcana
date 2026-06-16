//! Nicol Bolas, God-Pharaoh — `{4}{U}{B}{R}` Legendary Planeswalker — Bolas.
//! Starting loyalty inferred 7.
//! +2: Target opponent exiles cards from the top of their library until they
//!   exile a nonland card. Until end of turn, you may cast that card without
//!   paying its mana cost.
//! +1: Each opponent exiles two cards from their hand.
//! −4: Nicol Bolas deals 7 damage to target opponent, creature an opponent
//!   controls, or planeswalker an opponent controls.
//! −12: Exile each nonland permanent your opponents control.
//!
//! GAP: +2 "exile from top of library until a nonland, then you may cast it
//!   free this turn" — no exile-until-nonland-of-target-player + cast-from-
//!   exile-free rider in the demonstrated surface. Declared with the correct
//!   +2 cost and a target-opponent requirement, effect GAP'd.
//! GAP: +1 "each opponent exiles two cards from their hand" — no exile-from-
//!   hand effect in the demonstrated surface. Declared with the correct +1
//!   cost, effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nicol Bolas, God-Pharaoh");
    let bolas = reg.interner_mut().intern("Bolas");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bolas);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Target opponent exiles cards from the top of their library \
                       until they exile a nonland card. Until end of turn, you may cast \
                       that card without paying its mana cost.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each opponent exiles two cards from their hand.".into(),
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
                text: "-4: Nicol Bolas deals 7 damage to target opponent, creature an \
                       opponent controls, or planeswalker an opponent controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-12: Exile each nonland permanent your opponents control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 12)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_twelve,
            }),
    )
}

fn plus_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile-until-nonland of a target player's library + cast-free rider.
    Vec::new()
}

fn plus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: each opponent exiles two cards from their hand — no exile-from-hand.
    Vec::new()
}

fn minus_four_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 7,
    }]
}

fn minus_twelve(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::Opponent);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::ExilePermanent { target: id })
        .collect()
}
