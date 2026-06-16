//! Eternal Dragon — `{5}{W}{W}` 5/5 Creature — Dragon Spirit.
//! Flying.
//! "{3}{W}{W}: Return this card from your graveyard to your hand. Activate only
//! during your upkeep."
//! "Plainscycling {2} ({2}, Discard this card: Search your library for a Plains
//! card, ...)" — emitted as generic Cycling {2}.
//!
//! GAP: the graveyard return's "Activate only during your upkeep" timing
//! restriction is not expressible — the ability is unrestricted-timing.
//! GAP: Plainscycling's basic-land/type search fidelity (generic Cycling draws
//! a card instead of searching for a Plains).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eternal Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}{W}: Return this card from your graveyard to your hand. Activate only during your upkeep.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_to_hand,
        }),
    )
}

fn return_to_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Activate only during your upkeep" timing restriction is not
    // expressible — this ability is unrestricted-timing.
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
