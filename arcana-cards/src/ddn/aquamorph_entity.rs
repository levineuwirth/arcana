//! Aquamorph Entity — `{2}{U}{U}` */* Shapeshifter.
//! "As this creature enters or is turned face up, it becomes your choice of
//!  5/1 or 1/5." — GAP: no effect expresses a player-choice base-P/T set as
//!  a replacement on enter / turn-face-up.
//! "Morph {2}{U}" — GAP: Morph is outside the usable keyword surface for
//!  this card class.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aquamorph Entity");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    // GAP: "As this creature enters or is turned face up, it becomes your
    // choice of 5/1 or 1/5." — no expressible replacement/choice effect.
    // GAP: "Morph {2}{U}" — Morph is not in the usable keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
