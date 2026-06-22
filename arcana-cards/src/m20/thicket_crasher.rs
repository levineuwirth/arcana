//! Thicket Crasher — `{3}{G}` 4/3 green Creature — Elemental Rhino.
//!
//! Trample
//! Other Elementals you control have trample.
//!
//! Decomposition: Trample → `keywords`. "Other Elementals you control
//! have trample" is a pure static continuous ability (no trigger word, no
//! cost) granting a keyword board-wide — not expressible as a
//! triggered/activated ability, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thicket Crasher");
    let elemental = reg.interner_mut().intern("Elemental");
    let rhino = reg.interner_mut().intern("Rhino");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(rhino);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: static "Other Elementals you control have trample." — a
    // board-wide keyword-granting continuous ability with no trigger/cost;
    // not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
