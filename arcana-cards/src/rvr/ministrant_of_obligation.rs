//! Ministrant of Obligation — `{2}{W}` 2/1 Human Cleric with Afterlife 2.
//! When Ministrant of Obligation dies, create two 1/1 white and black
//! Spirit creature tokens with flying. (CR 702.151 — Afterlife.)
//!
//! # Rules references
//!
//! * CR 702.151 — Afterlife N. When this permanent dies, create N 1/1
//!   white and black Spirit creature tokens with flying.
//!
//! Afterlife is a fully implemented parametrized keyword; listing
//! `KeywordAbility::Afterlife(2)` in `keywords` is sufficient — the
//! runtime pipeline handles token creation on death.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ministrant of Obligation");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Afterlife(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
