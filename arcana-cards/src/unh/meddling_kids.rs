//! Meddling Kids — `{2}{W}{U}` 2/3 Human Child.
//! As this creature enters, choose a word with four or more letters.
//! Spells with the chosen word in their text box can't be cast.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meddling Kids");
    let human = reg.interner_mut().intern("Human");
    let child = reg.interner_mut().intern("Child");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(child);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "choose a word" + "spells with the chosen word can't be cast" —
    // a text-introspecting static cast-restriction, not expressible with any
    // Effect/static.
    reg.register(CardDefinition::new(name, chars))
}
