//! Nicol Bolas, Dragon-God — `{U}{B}{B}{B}{R}` Legendary Planeswalker — Bolas,
//! starting loyalty 7.
//!
//! Static: Nicol Bolas has all loyalty abilities of all other planeswalkers on
//!   the battlefield.
//! +1: You draw a card. Each opponent exiles a card from their hand or a
//!   permanent they control.
//! −3: Destroy target creature or planeswalker.
//! −8: Each opponent who doesn't control a legendary creature or planeswalker
//!   loses the game.
//!
//! GAP: the static "has all loyalty abilities of all other planeswalkers" is a
//!   continuous ability-granting effect, not a loyalty ability — not declared.
//! GAP: the +1's "each opponent exiles a card from their hand or a permanent
//!   they control" is a choice-driven exile not in the demonstrated Effect
//!   surface; only the "you draw a card" half is emitted.
//! GAP: the −8 "each opponent … loses the game" — no Effect variant for
//!   making a player lose the game; declared with its −8 cost, effect empty.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Nicol Bolas, Dragon-God");
    let bolas = reg.interner_mut().intern("Bolas");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bolas);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{B}{B}{R}").expect("valid cost")),
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
                text: "+1: You draw a card. Each opponent exiles a card from \
                       their hand or a permanent they control.".into(),
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
                effect: plus_one_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target creature or planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Each opponent who doesn't control a legendary \
                       creature or planeswalker loses the game.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_lose,
            }),
    )
}

/// `+1: You draw a card. …`
fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent exiles a card from their hand or a permanent they
    // control" is a choice-driven exile not in the demonstrated surface; only
    // the draw is emitted.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

/// `−3: Destroy target creature or planeswalker.`
fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−8: Each opponent … loses the game.`
fn minus_eight_lose(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant for "a player loses the game".
    Vec::new()
}
