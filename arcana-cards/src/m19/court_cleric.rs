//! Court Cleric — `{W}` 1/1 Human Cleric with Lifelink.
//!
//! Oracle:
//! * Lifelink.
//! * This creature gets +1/+1 as long as you control an Ajani planeswalker.
//!   (conditional static self-buff — GAP: not expressible.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Court Cleric");
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
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: "This creature gets +1/+1 as long as you control an Ajani
    // planeswalker." — conditional static P/T buff is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
