//! Domri, Chaos Bringer — `{2}{R}{G}` Legendary Planeswalker — Domri, loyalty 4.
//!
//! +1: Add {R} or {G}. If that mana is spent on a creature spell, it gains riot.
//! −3: Look at the top four cards of your library. You may reveal up to two
//!   creature cards from among them and put them into your hand. Put the rest on
//!   the bottom of your library in a random order.
//! −8: You get an emblem with "At the beginning of each end step, create a 4/4
//!   red and green Beast creature token with trample."
//!
//! # Scope
//! GAP: the +1 "{R} or {G}" color choice and the "if spent on a creature spell
//!   it gains riot" rider aren't expressible — modeled as adding {R}.
//! GAP: the −3 "up to TWO" creature picks exceed DigTopN's single-pick surface;
//!   modeled as a single optional creature pick (DigTopN), rest to bottom.
//! GAP: the −8 emblem (recurring each-end-step Beast token) is an emblem-borne
//!   triggered ability not expressible. Ability shell declared, effect empty.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaUnit, ManaCost};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::effects::DigRest;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri, Chaos Bringer");
    let domri = reg.interner_mut().intern("Domri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(domri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R} or {G}. If that mana is spent on a creature spell, it gains riot.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Look at the top four cards of your library. You may reveal up to two creature cards from among them and put them into your hand. Put the rest on the bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"At the beginning of each end step, create a 4/4 red and green Beast creature token with trample.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_gap,
            }),
    )
}

fn plus_one_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: {R}-or-{G} color choice and the "spent on a creature spell → riot"
    // rider aren't expressible; adding {R}.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn minus_three_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "up to TWO" creature picks; modeled as a single optional creature pick.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_eight_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with recurring each-end-step 4/4 Beast token.
    Vec::new()
}
