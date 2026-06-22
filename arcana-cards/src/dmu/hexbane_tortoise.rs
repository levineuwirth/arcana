//! Hexbane Tortoise — `{2}{G}` 3/2 Turtle with Ward {2} and Enlist.
//! Ward {2}.
//! Enlist.
//!
//! Both abilities are keyword-line entries fully supported by the usable
//! keyword surface: Ward {2} is `KeywordAbility::Ward(ManaCost)` and Enlist is
//! `KeywordAbility::Enlist`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hexbane Tortoise");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
            KeywordAbility::Enlist,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
