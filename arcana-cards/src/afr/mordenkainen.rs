//! Mordenkainen — `{4}{U}{U}` legendary planeswalker, starting loyalty 5.
//! Subtype Mordenkainen; mono-blue.
//!
//! Loyalty abilities:
//! * `+2`: Draw two cards, then put a card from your hand on the bottom of
//!   your library. (The draw is functional; the "put a card from hand on
//!   the bottom" is omitted — no hand-to-bottom-of-library effect.)
//! * `−2`: Create a blue Dog Illusion token whose P/T equals twice your
//!   hand size. GAP — a dynamic-P/T (CDA) token isn't expressible.
//! * `−10`: Exchange your hand and library, then shuffle; emblem. GAP —
//!   hand/library swap plus emblem creation are bespoke.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mordenkainen");
    let mordenkainen = reg.interner_mut().intern("Mordenkainen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mordenkainen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
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
                text: "+2: Draw two cards, then put a card from your hand on \
                       the bottom of your library."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a blue Dog Illusion creature token with \
                       \"This token's power and toughness are each equal to \
                       twice the number of cards in your hand.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Exchange your hand and library, then shuffle. You \
                       get an emblem with \"You have no maximum hand size.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_gap,
            }),
    )
}

fn plus_two_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // The draw is functional; "put a card from hand on the bottom" is
    // omitted (no hand-to-bottom-of-library effect).
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}

fn minus_two_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic-P/T (CDA) token equal to twice hand size isn't
    // expressible.
    Vec::new()
}

fn minus_ten_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: hand/library exchange plus emblem creation are bespoke.
    Vec::new()
}
