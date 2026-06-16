//! Jace, the Perfected Mind — `{2}{U}{U/P}` Legendary Planeswalker — Jace,
//! printed loyalty 5. Colors U.
//!
//! Compleated ({U/P} can be paid with {U} or 2 life; if life was paid, this
//! planeswalker enters with two fewer loyalty counters.)
//! +1: Until your next turn, up to one target creature gets -3/-0.
//! −2: Target player mills three cards. Then if a graveyard has twenty or more
//!   cards in it, you draw three cards. Otherwise, you draw a card.
//! −X: Target player mills three times X cards.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.
//! * CR 113.3c — entering with loyalty counters.
//!
//! # Scope / GAPs
//! * Compleated (the {U/P} life-payment that reduces entering loyalty by 2) is
//!   cost / enters-with machinery, NOT a loyalty ability — not modeled here;
//!   loyalty set to the printed 5. GAP.
//! * −2: the "if a graveyard has twenty or more cards, draw three, otherwise
//!   draw one" conditional isn't expressible (no any-graveyard count Condition
//!   in the demonstrated surface). The common branch (draw one) is modeled; the
//!   20+ "draw three" branch is GAP.
//! * −X uses the dynamic-X loyalty cost (`remove_loyalty_x`); the engine threads
//!   the chosen X and mills 3 * X.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, the Perfected Mind");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U/P}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target creature gets \
                       -3/-0.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Target player mills three cards. Then if a graveyard \
                       has twenty or more cards in it, you draw three cards. \
                       Otherwise, you draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-X: Target player mills three times X cards.".into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x,
            }),
    )
}

/// `+1: Until your next turn, up to one target creature gets -3/-0.`
fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -3,
        toughness: 0,
        duration: Duration::UntilYourNextTurn(ctx.controller),
        keywords: vec![],
    }]
}

/// `-2: Target player mills three cards; then draw (the 20+-graveyard branch is
/// GAP, so always the "draw a card" branch).`
fn minus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: the "if a graveyard has twenty or more cards, you draw three cards"
    //      branch needs an any-graveyard size Condition not in the demonstrated
    //      surface. Model the common case: mill 3, then draw one.
    vec![
        Effect::Mill {
            player: *p,
            count: 3,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
    ]
}

/// `-X: Target player mills three times X cards.`
fn minus_x(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = ctx.x_value.unwrap_or(0);
    vec![Effect::Mill {
        player: *p,
        count: 3 * x,
    }]
}
