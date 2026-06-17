//! Slipstream Eel — `{5}{U}{U}` 6/6 Fish Beast with Cycling {1}{U}.
//!
//! Oracle:
//! * This creature can't attack unless defending player controls an Island.
//!   (static attack restriction — GAP: not expressible.)
//! * Cycling {1}{U}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slipstream Eel");
    let fish = reg.interner_mut().intern("Fish");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{1}{U}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: "This creature can't attack unless defending player controls an
    // Island." — conditional attack restriction is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
