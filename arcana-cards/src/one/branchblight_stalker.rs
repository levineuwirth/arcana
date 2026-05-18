//! Branchblight Stalker — `{1}{G}` 3/1 Phyrexian Elf Scout with Toxic 2.
//! March of the Machine common; a green Toxic 2 creature at 2 mana.
//!
//! # Rules references
//!
//! * CR 702.164 — Toxic N. Players dealt combat damage by this creature
//!   also get N poison counters (in addition to any other effects).
//!   Engine wiring handles the additional poison counter assignment in
//!   the combat damage pipeline.
//!
//! Toxic is a fully implemented parametrized keyword in the engine;
//! `KeywordAbility::Toxic(2)` is sufficient.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Branchblight Stalker");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Toxic(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
