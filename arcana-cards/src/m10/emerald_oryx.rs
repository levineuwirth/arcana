//! Emerald Oryx — `{3}{G}` 2/3 Antelope with Forestwalk.
//! Mirage common; a green Antelope that is unblockable in the mirror
//! match where opponents control Forests.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Forest subtype). This creature can't be
//!   blocked as long as defending player controls a Forest.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Forest"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emerald Oryx");
    let antelope = reg.interner_mut().intern("Antelope");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(antelope);

    let forest = reg.interner_mut().intern("Forest");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
