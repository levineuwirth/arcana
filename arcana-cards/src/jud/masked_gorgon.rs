//! Masked Gorgon — `{4}{B}` 5/5 Gorgon.
//! Green creatures and white creatures have protection from Gorgons. (static — GAP)
//! Threshold — This creature has protection from green and from white as
//! long as there are seven or more cards in your graveyard. (static / Threshold — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Masked Gorgon");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);

    // GAP: "Green and white creatures have protection from Gorgons" — a
    // continuous static granting protection to other permanents.
    // GAP: Threshold static "~ has protection from green and white while you
    // have 7+ cards in graveyard" — Threshold is not a supported keyword and
    // protection is not an expressible effect; no triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
