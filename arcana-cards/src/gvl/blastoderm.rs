//! Blastoderm — `{2}{G}{G}` 5/5 green Beast with Shroud and Fading 3.
//! Both keywords are engine-handled (Shroud as a targeting restriction; Fading 3
//! seeds three fade counters and the upkeep remove-or-sacrifice loop).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blastoderm");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Shroud, KeywordAbility::Fading(3)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
