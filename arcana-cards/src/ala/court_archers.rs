//! Court Archers — `{2}{G}` 1/3 Human Archer with Reach and Exalted.
//!
//! Both abilities are evergreen keyword abilities parsed by Scryfall:
//! Reach (can block fliers) and Exalted (whenever a creature you
//! control attacks alone, that creature gets +1/+1 until end of turn).
//! Both are base characteristics, so listing them in `keywords` is all
//! that's required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Court Archers");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Exalted],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
