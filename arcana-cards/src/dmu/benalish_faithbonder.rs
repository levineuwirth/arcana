//! Benalish Faithbonder — `{1}{W}` 1/3 Human Cleric with Vigilance and Enlist.
//!
//! * Vigilance.
//! * Enlist.
//!
//! Both keywords are in the usable KeywordAbility surface, so the card is a
//! pure keyword-line creature.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benalish Faithbonder");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Enlist],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
