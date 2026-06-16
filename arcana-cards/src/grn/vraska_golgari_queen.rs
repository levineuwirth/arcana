//! Vraska, Golgari Queen — `{2}{B}{G}` Legendary Planeswalker — Vraska, starting loyalty 4.
//!
//! +2: You may sacrifice another permanent. If you do, you gain 1 life and draw a
//!     card. The optional sacrifice-cost gating its payoff is not expressible from
//!     the demonstrated surface (sacrifice is not an OptionalPaymentKind) — GAP
//!     (effect returns empty; +2 shell preserved).
//! −3: Destroy target nonland permanent with mana value 3 or less. Fully modeled.
//! −9: You get an emblem with "Whenever a creature you control deals combat damage
//!     to a player, that player loses the game." A rule-altering player-loses-the-
//!     game triggered effect the builders can't express — GAP (−9 shell preserved,
//!     emblem omitted).

use arcana_core::effects::Effect;
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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Golgari Queen");
    let vraska = reg.interner_mut().intern("Vraska");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let cheap_nonland = ObjectFilter::new()
        .without_types(TypeLine::LAND.into())
        .with_max_cmc(3);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You may sacrifice another permanent. If you do, you gain \
                       1 life and draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_sacrifice,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target nonland permanent with mana value 3 or \
                       less.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(cheap_nonland),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: You get an emblem with \"Whenever a creature you control \
                       deals combat damage to a player, that player loses the \
                       game.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_emblem,
            }),
    )
}

fn plus_two_sacrifice(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "You may sacrifice another permanent. If you do, you gain 1 life and
    //      draw a card." — an optional sacrifice cost gating its payoff; sacrifice
    //      is not an OptionalPaymentKind and the if-you-do conditional bundle is
    //      not expressible from the demonstrated surface.
    Vec::new()
}

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

fn minus_nine_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem "Whenever a creature you control deals combat damage to a
    //      player, that player loses the game." — a rule-altering "loses the
    //      game" triggered effect the demonstrated builders can't express.
    Vec::new()
}
