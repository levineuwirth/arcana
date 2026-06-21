//! Two-Headed Giant of Foriys — `{4}{R}` 4/4 Creature — Giant.
//! Trample. "This creature can block an additional creature each combat."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Two-Headed Giant of Foriys");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    // GAP: static "can block an additional creature each combat" — no
    // modeled primitive for raising the block count.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
