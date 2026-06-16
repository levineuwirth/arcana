//! Wrenn and Seven — `{3}{G}{G}` Legendary Planeswalker — Wrenn.
//! Starting loyalty 5 (oracle).
//!
//! +1: Reveal the top four cards of your library. Put all land cards revealed
//!     this way into your hand and the rest into your graveyard.
//!     GAP: "reveal top N, put ALL matching into hand, rest into graveyard"
//!     is not expressible (RevealUntil takes only the FIRST match; DigTopN
//!     takes a single optional pick).
//! 0: Put any number of land cards from your hand onto the battlefield tapped.
//!     GAP: "any number of cards from hand onto battlefield" is not
//!     expressible (PutFromHandOntoBattlefield posts a single 0/1 pick).
//! −3: Create a green Treefolk creature token with reach and "*/* equal to
//!     lands you control". GAP: a token whose P/T is a dynamic
//!     board-derived characteristic-defining ability is not expressible.
//! −8: Return all permanent cards from your graveyard to your hand. You get
//!     an emblem with "You have no maximum hand size."
//!     GAP: "return ALL permanent cards from graveyard" (no all-graveyard
//!     filter return) and emblem creation not modeled.

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
    let name = reg.interner_mut().intern("Wrenn and Seven");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top four cards of your library. Put all \
                       land cards into your hand and the rest into your \
                       graveyard.".into(),
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
                effect: gap_effect,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Put any number of land cards from your hand onto the \
                       battlefield tapped.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_effect,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Create a green Treefolk creature token with reach \
                       and \"P/T equal to lands you control\".".into(),
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
                effect: gap_effect,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Return all permanent cards from your graveyard to \
                       your hand. You get an emblem.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_effect,
            }),
    )
}

fn gap_effect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: see doc comment — each of Wrenn and Seven's abilities relies on a
    //      primitive not in the demonstrated surface (put-all-matching to a
    //      zone, put-any-number-from-hand, dynamic-P/T token, return-all-
    //      from-graveyard, emblem).
    Vec::new()
}
