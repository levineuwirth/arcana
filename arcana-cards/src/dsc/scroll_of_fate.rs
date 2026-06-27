//! Scroll of Fate — {3} artifact.
//! "{T}: Manifest a card from your hand. (Put that card onto the battlefield
//! face down as a 2/2 creature. Turn it face up any time for its mana cost if
//! it's a creature card.)"
//!
//! GAP (fidelity): `Effect::Manifest` manifests the TOP CARD OF THE LIBRARY,
//! whereas this card manifests a chosen card from hand. The closest available
//! primitive is used; the from-hand source is unmodeled. The Scryfall keyword
//! `Manifest` has no `KeywordAbility` variant, so `keywords` is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scroll of Fate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Manifest a card from your hand.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: manifest_from_hand,
        }),
    )
}

fn manifest_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): manifests the top of library, not a chosen card from hand
    // (no "manifest from hand" primitive exists).
    vec![Effect::Manifest { player: ctx.controller }]
}
