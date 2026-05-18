//! Cystbearer — `{2}{G}` 2/3 Phyrexian Beast with Infect.
//! Scars of Mirrodin common; a green Infect creature at 3 mana.
//!
//! # Rules references
//!
//! * CR 702.90 — Infect. This creature deals damage to creatures in the
//!   form of -1/-1 counters and to players in the form of poison counters.
//!   Engine wiring handles the counter substitution in the damage-dealing
//!   pipeline.
//!
//! Infect is a fully implemented keyword in the engine; listing it in
//! `keywords` is sufficient.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cystbearer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
