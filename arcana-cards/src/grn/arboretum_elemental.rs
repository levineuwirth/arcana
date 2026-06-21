//! Arboretum Elemental — `{7}{G}{G}` 7/5 Creature — Elemental.
//!
//! Oracle:
//! * Convoke (cast-cost reduction; NOT in the usable keyword surface — GAP'd)
//! * Hexproof
//!
//! Convoke is a casting-cost mechanic with no `KeywordAbility` variant and
//! no expressible primitive, so it is GAP'd. Hexproof is an evergreen
//! keyword and is emitted normally.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arboretum Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    // GAP: Convoke — casting-cost reduction mechanic, no KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
