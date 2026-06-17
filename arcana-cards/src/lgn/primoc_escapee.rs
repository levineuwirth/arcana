//! Primoc Escapee — `{6}{U}` 4/4 Bird Beast with Flying and Cycling {2}.
//! Flying
//! Cycling {2}

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Primoc Escapee");
    let bird = reg.interner_mut().intern("Bird");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
