//! Scourge Servant — `{4}{B}` 3/3 Phyrexian Zombie with Infect.
//!
//! # Rules references
//!
//! * CR 702.90 — Infect. This creature deals damage to creatures in
//!   the form of -1/-1 counters and to players in the form of poison
//!   counters.
//!
//! NOTE: `Infect` is not present in the demonstrated `KeywordAbility`
//! variant list. The `keywords` field is left empty so this file
//! compiles; the verify pipeline will flag the missing variant and a
//! human will route it.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge Servant");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
