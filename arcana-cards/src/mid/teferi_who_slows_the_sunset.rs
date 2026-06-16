//! Teferi, Who Slows the Sunset — `{2}{W}{U}` Legendary Planeswalker — Teferi.
//! Starting loyalty 5 (oracle).
//!
//! +1: Choose up to one target artifact, up to one target creature, and up
//!     to one target land. Untap the chosen permanents you control. Tap the
//!     chosen permanents you don't control. You gain 2 life.
//!     PARTIAL: the "you gain 2 life" rider is modeled. GAP: the three
//!     up-to-one targets with the per-target untap-if-you-control /
//!     tap-if-you-don't conditional resolution is not expressible.
//! −2: Look at the top three cards of your library. Put one of them into your
//!     hand and the rest on the bottom of your library in any order.
//! −7: You get an emblem. GAP: emblem creation not modeled.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Who Slows the Sunset");
    let teferi = reg.interner_mut().intern("Teferi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teferi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose up to one target artifact, creature, and \
                       land. Untap the chosen permanents you control, tap \
                       those you don't. You gain 2 life.".into(),
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
                effect: plus_one_gain_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Look at the top three cards of your library. Put \
                       one into your hand and the rest on the bottom in any \
                       order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem.".into(),
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
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_gain_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: only the "you gain 2 life" rider is expressed.
    // GAP: up-to-one artifact/creature/land targets with conditional
    //      untap-yours / tap-not-yours resolution not expressible.
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: 2,
    }]
}

fn minus_two_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with "Untap all permanents you control during each
    //      opponent's untap step" and "You draw a card during each
    //      opponent's draw step" — emblem creation not modeled.
    Vec::new()
}
