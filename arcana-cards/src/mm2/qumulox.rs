//! Qumulox — `{6}{U}{U}` 5/4 Beast with Affinity for artifacts and
//! Flying.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Qumulox");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Affinity for artifacts (cost reduction) is not in the usable
        // KeywordAbility surface; only Flying is emitted.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
