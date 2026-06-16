//! Narset, Parter of Veils — `{1}{U}{U}` Legendary Planeswalker — Narset,
//! starting loyalty 5.
//!
//! Static (GAP): Each opponent can't draw more than one card each turn.
//! (Not a loyalty ability; a per-turn draw-limit replacement not in the
//! demonstrated surface — omitted.)
//! −2: Look at the top four cards of your library. You may reveal a
//!     noncreature, nonland card from among them and put it into your hand.
//!     Put the rest on the bottom of your library in a random order.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset, Parter of Veils");
    let narset = reg.interner_mut().intern("Narset");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(narset);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
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
                text: "-2: Look at the top four cards of your library. You may reveal a noncreature, nonland card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.".into(),
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
            }),
    )
}

fn minus_two_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(ObjectFilter {
            not_types: Some(TypeLine(TypeLine::CREATURE | TypeLine::LAND)),
            ..Default::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
