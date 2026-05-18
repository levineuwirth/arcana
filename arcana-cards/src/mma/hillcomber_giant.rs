//! Hillcomber Giant — `{2}{W}{W}` 3/3 Giant Scout with Mountainwalk.
//! A white giant scout whose Mountainwalk (a Landwalk variant) is not
//! yet representable in the engine's demonstrated API; the keyword list
//! is left empty as a best-effort stub for the verify pipeline.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk / Mountainwalk. (Not yet wired; verify pipeline will flag.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hillcomber Giant");
    let giant = reg.interner_mut().intern("Giant");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
