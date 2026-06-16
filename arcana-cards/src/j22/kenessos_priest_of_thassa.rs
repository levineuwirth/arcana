//! Kenessos, Priest of Thassa — `{1}{U}` 1/3 Legendary Merfolk Cleric.
//! If you would scry a number of cards, scry that many plus one instead
//! (static replacement — GAP).
//! `{3}{G/U}: Look at the top card of your library. If it's a Kraken,
//! Leviathan, Octopus, or Serpent creature card, you may put it onto the
//! battlefield. If you don't, you may put it on the bottom of your library.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kenessos, Priest of Thassa");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static scry-amount replacement ("scry that many plus one instead")
    // has no engine Effect / replacement primitive in the documented surface.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G/U}: Look at the top card of your library. If it's a Kraken, Leviathan, \
                   Octopus, or Serpent creature card, you may put it onto the battlefield. If you \
                   don't, you may put it on the bottom of your library."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G/U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: look_top_put,
        }),
    )
}

fn look_top_put(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top card; if it's a [tribal] creature card you may put
    // it onto the battlefield, else may put it on the bottom" is a conditional
    // top-of-library-to-battlefield put with no matching documented Effect
    // (DigTopN puts the chosen card to hand, not the battlefield).
    Vec::new()
}
