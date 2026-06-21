//! The Ninth Doctor — `{1}{U}{R}` 2/4 Legendary Time Lord Doctor with Haste.
//!
//! Oracle:
//! * Haste (keyword line).
//! * Into the TARDIS — Whenever The Ninth Doctor becomes untapped during
//!   your untap step, you get an additional upkeep step after this step.
//!   GAP: there is no "becomes untapped" trigger condition and no
//!   "additional upkeep step" effect — the ability is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Ninth Doctor");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(doctor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
