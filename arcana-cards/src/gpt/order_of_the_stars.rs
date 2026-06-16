//! Order of the Stars — `{W}` 0/1 Human Cleric with Defender.
//! "As this creature enters, choose a color. This creature has
//! protection from the chosen color."
//!
//! Defender is a base keyword. The choose-a-color ETB and the
//! resulting protection-from-chosen-color static are not expressible
//! with the demonstrated API.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Order of the Stars");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "As this creature enters, choose a color." — choose-a-color
    // ETB replacement is not expressible with the demonstrated API.
    // GAP: "This creature has protection from the chosen color." —
    // dynamic protection-from-chosen-color static is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
