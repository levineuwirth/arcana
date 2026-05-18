//! Dwarven Grunt — `{R}` 1/1 Dwarf with Mountainwalk.
//! A small red Dwarf that cannot be blocked while the defending
//! player controls a Mountain.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Mountainwalk). The creature is unblockable
//!   while the defending player controls a land of the named type.
//!
//! NOTE: Mountainwalk is a Landwalk variant. The current API's
//! `KeywordAbility` enum does not expose a Landwalk/Mountainwalk
//! variant in the demonstrated examples. The keyword is omitted here;
//! the verify pipeline should route this for manual wiring.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Grunt");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
