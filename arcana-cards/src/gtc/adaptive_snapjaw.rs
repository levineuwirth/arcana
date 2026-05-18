//! Adaptive Snapjaw — `{4}{G}` 6/2 Lizard Beast with Evolve.
//! Gatecrash common; a Simic evolve creature that grows when larger
//! creatures enter under your control.
//!
//! # Rules references
//!
//! * CR 702.100 — Evolve. Whenever a creature you control enters, if
//!   that creature has greater power or toughness than this creature,
//!   put a +1/+1 counter on this creature. Engine wiring handles the
//!   triggered comparison and counter placement.
//!
//! The keyword is a base characteristic; listing it in `keywords` is
//! sufficient — the runtime pipeline does the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adaptive Snapjaw");
    let lizard = reg.interner_mut().intern("Lizard");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
