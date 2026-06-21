//! Achilles Davenport — `{2}{U}{B}` 3/3 Legendary Human Assassin.
//!
//! * Freerunning {U}{B}. (Alternative-cost keyword — not modeled; GAP.)
//! * Menace.
//! * Other Assassins you control get +1/+1. (Static anthem — GAP.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Achilles Davenport");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    // GAP: Freerunning {U}{B} is an alternative-cost casting keyword with no
    // KeywordAbility variant — omitted.
    // GAP: "Other Assassins you control get +1/+1" is a static anthem.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
