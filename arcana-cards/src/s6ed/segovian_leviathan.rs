//! Segovian Leviathan — `{4}{U}` 3/3 Leviathan with Islandwalk.
//! Legends uncommon; a blue sea creature that can't be blocked as
//! long as the defending player controls an Island.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. This creature can't be blocked as long
//!   as the defending player controls an Island.
//!   Engine wiring: `KeywordAbility::Landwalk("Island")` is checked
//!   by the combat blocker filter against the defending player's lands.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Segovian Leviathan");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(island)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
