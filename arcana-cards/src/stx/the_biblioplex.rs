//! The Biblioplex — nonbasic land.
//! "{T}: Add {C}." and "{2}, {T}: Look at the top card of your library.
//! If it's an instant or sorcery card, you may reveal it and put it into
//! your hand. If you don't put the card into your hand, you may put it
//! into your graveyard. Activate only if you have exactly zero or seven
//! cards in hand."
//!
//! Modeled with `Effect::DigTopN` (count 1, instant-or-sorcery filter,
//! rest to graveyard).
//! GAP: "Activate only if you have exactly zero or seven cards in hand"
//! — there is no activation-precondition field on `ActivationCost`.
//! GAP: a non-taken card MAY go to the graveyard (else stays on top);
//! `DigRest::Graveyard` sends it there unconditionally.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Biblioplex");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Look at the top card of your library. If it's an instant or sorcery card, you may reveal it and put it into your hand. If you don't put the card into your hand, you may put it into your graveyard. Activate only if you have exactly zero or seven cards in hand.".into(),
                // GAP: "Activate only if you have exactly zero or seven cards
                // in hand" — no activation precondition is expressible.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: dig_top,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn dig_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the non-taken card MAY go to the graveyard (else stays on top);
    // DigRest::Graveyard sends it there unconditionally.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(
            ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        ),
        rest: DigRest::Graveyard,
    }]
}
