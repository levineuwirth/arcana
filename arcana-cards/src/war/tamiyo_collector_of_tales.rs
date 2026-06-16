//! Tamiyo, Collector of Tales — `{2}{G}{U}` Legendary Planeswalker — Tamiyo.
//! Starting loyalty inferred 4.
//! Static: Spells and abilities your opponents control can't cause you to
//!   discard cards or sacrifice permanents.
//! +1: Choose a nonland card name, then reveal the top four cards of your
//!   library. Put all cards with the chosen name from among them into your
//!   hand and the rest into your graveyard.
//! −3: Return target card from your graveyard to your hand.
//!
//! GAP: the "can't cause you to discard/sacrifice" static is a continuous
//!   replacement-style restriction, not a loyalty ability — not modeled here
//!   (no loyalty cost).
//! GAP: +1 "choose a nonland card name, reveal top four, matching → hand, rest
//!   → graveyard" — no choose-a-card-name-then-filter-top-N primitive in the
//!   demonstrated surface. Declared with correct +1 cost, effect GAP'd.
//! GAP: −3 targets a card in YOUR graveyard — controller-relative graveyard
//!   targeting has no sentinel in the demonstrated surface. Declared, GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Tamiyo, Collector of Tales");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose a nonland card name, then reveal the top four cards \
                       of your library. Put all cards with the chosen name from among \
                       them into your hand and the rest into your graveyard.".into(),
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
                effect: plus_one_name,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return target card from your graveyard to your hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_return,
            }),
    )
}

fn plus_one_name(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: choose-a-card-name then partition the top four by that name into
    // hand/graveyard — no such primitive in the demonstrated surface.
    Vec::new()
}

fn minus_three_return(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target card from your graveyard" — controller-relative graveyard
    // targeting has no sentinel in the demonstrated surface.
    Vec::new()
}
