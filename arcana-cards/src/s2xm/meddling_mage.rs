//! Meddling Mage — `{W}{U}` 2/2 Creature — Human Wizard.
//!
//! Oracle:
//! * As this creature enters, choose a nonland card name.
//! * Spells with the chosen name can't be cast.
//!
//! The as-enters name choice plus the static cast-prohibition for that name
//! have no matching primitive (NameCardAndExile strips a deck, not "can't be
//! cast"; there is no cast-prohibition-by-name effect), so both are GAP'd and
//! only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meddling Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "As this creature enters, choose a nonland card name; spells with the
    // chosen name can't be cast." — no choose-a-name + cast-prohibition
    // mechanic in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
