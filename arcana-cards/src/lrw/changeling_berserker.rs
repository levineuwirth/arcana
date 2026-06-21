//! Changeling Berserker — `{3}{R}` 5/3 red Shapeshifter.
//! Changeling, Haste, Champion a creature.
//!
//! Changeling and Haste are wired as keywords. "Champion a creature"
//! (When this enters, sacrifice it unless you exile another creature
//! you control; when this leaves, that card returns) is the Champion
//! mechanic — there is no Champion KeywordAbility variant and no
//! exile-as-cost / linked-return effect available, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Changeling Berserker");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    // GAP: "Champion a creature" — the Champion mechanic (exile-as-ETB,
    // sacrifice-unless, linked leave-return) has no available primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Changeling],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
