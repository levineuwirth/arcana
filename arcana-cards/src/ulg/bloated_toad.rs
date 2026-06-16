//! Bloated Toad — `{2}{G}` 2/2 Frog.
//! Protection from blue (GAP — Protection is not in the usable keyword surface).
//! Cycling {2} (engine synthesizes the "{2}, discard: draw a card" ability).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloated Toad");
    let frog = reg.interner_mut().intern("Frog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Protection from blue" — Protection is not an available keyword.
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
