//! Imperious Oligarch — `{W}{B}` 2/1 Human Cleric.
//! "Vigilance"
//! "Afterlife 1 (When this creature dies, create a 1/1 white and black
//! Spirit creature token with flying.)"
//!
//! Both Vigilance and the parametrized Afterlife keyword are in the
//! usable keyword surface; the engine synthesizes the Afterlife
//! dies-trigger from the keyword, so no triggered ability is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imperious Oligarch");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Afterlife(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
