//! Riptide Turtle — `{1}{U}` 0/5 Turtle with Flash and Defender.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riptide Turtle");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
