//! Hullbreacher — `{2}{U}` 3/2 Merfolk Pirate with Flash.
//! "If an opponent would draw a card except the first one they draw in each
//!  of their draw steps, instead you create a Treasure token."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hullbreacher");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: replacement effect "if an opponent would draw a card (except the first
    // each draw step), instead you create a Treasure token" — no draw-replacement
    // Effect/installer is exposed for this card class.
    reg.register(CardDefinition::new(name, chars))
}
