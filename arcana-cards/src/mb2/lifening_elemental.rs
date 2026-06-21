//! Lifening Elemental — `{5}{B}` 4/6 black Vampire Elemental.
//!
//! Oracle:
//! * Lifelink
//! * Splice onto instant or sorcery {1}{B}
//!
//! Splice is not in the usable keyword surface for this card class
//! (no `KeywordAbility::Splice` variant is demonstrated), so it is
//! GAP'd. Only Lifelink is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lifening Elemental");
    let vampire = reg.interner_mut().intern("Vampire");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(elemental);

    // GAP: "Splice onto instant or sorcery {1}{B}" — Splice is not in
    // the usable keyword surface for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
