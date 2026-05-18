//! Gaea's Skyfolk — `{G}{U}` 2/2 Elf Merfolk with Flying.
//! Apocalypse common; a two-color creature bridging green and blue,
//! notable as one of the few green creatures with flying.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach. Engine wiring lives in [`arcana_core::combat`]'s
//!   blocker filter.
//!
//! Colors: G, U.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gaea's Skyfolk");
    let elf = reg.interner_mut().intern("Elf");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(merfolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
