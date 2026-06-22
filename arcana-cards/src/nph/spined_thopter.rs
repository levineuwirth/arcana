//! Spined Thopter — `{2}{U/P}` 2/1 Phyrexian Thopter Artifact Creature
//! with Flying. ({U/P} can be paid with either {U} or 2 life.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spined Thopter");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(thopter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U/P}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
