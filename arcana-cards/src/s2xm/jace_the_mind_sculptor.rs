//! Jace, the Mind Sculptor — `{2}{U}{U}` Legendary Planeswalker — Jace, loyalty 3.
//!
//! +2: Look at the top card of target player's library. You may put that card on
//!   the bottom of that player's library.
//! 0: Draw three cards, then put two cards from your hand on top of your library
//!   in any order.
//! −1: Return target creature to its owner's hand.
//! −12: Exile all cards from target player's library, then that player shuffles
//!   their hand into their library.
//!
//! # Scope
//! GAP: the +2 "fateseal" (look at top of target player's library, may bottom
//!   it) has no expressible effect. Ability shell declared, effect empty.
//! GAP: the 0 ability (draw three, then put two from hand on top of library) —
//!   the "put back two" half isn't expressible, so modeling only the draw would
//!   net the wrong cards. GAP'd whole. Ability shell declared, effect empty.
//! GAP: the −12 (mass-library exile + shuffle hand into library) has no
//!   expressible primitive. Ability shell declared, effect empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, the Mind Sculptor");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Look at the top card of target player's library. You may put that card on the bottom of that player's library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Draw three cards, then put two cards from your hand on top of your library in any order.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Return target creature to its owner's hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_bounce,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-12: Exile all cards from target player's library, then that player shuffles their hand into their library.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 12)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_twelve_gap,
            }),
    )
}

fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: fateseal (look at top of target player's library, may bottom it).
    Vec::new()
}

fn zero_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Brainstorm-style "draw three, put two from hand on top" — the
    // put-back half isn't expressible; modeling only the draw is unfaithful.
    Vec::new()
}

fn minus_one_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}

fn minus_twelve_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mass-library exile + shuffle hand into library.
    Vec::new()
}
