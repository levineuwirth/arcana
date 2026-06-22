//! Thunderscape Familiar — `{1}{R}` 1/1 Kavu with First strike.
//! First strike.
//! Black spells and green spells you cast cost {1} less to cast.
//!
//! First strike is a base characteristic. The cost-reduction line is a static
//! continuous ability (no trigger word, no cost) with no expressible primitive
//! for spell cost reduction, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunderscape Familiar");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    // GAP: static "Black spells and green spells you cast cost {1} less to
    // cast" — spell cost reduction is not an expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
