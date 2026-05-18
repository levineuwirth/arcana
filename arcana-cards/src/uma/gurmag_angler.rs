//! Gurmag Angler — `{6}{B}` 5/5 Zombie Fish with Delve.
//! Delve is not expressible with the current demonstrated KeywordAbility API;
//! the verify pipeline should flag this for manual wiring.
//!
//! # Rules references
//!
//! * CR 702.65 — Delve. Each card exiled from the graveyard while casting
//!   this spell pays for {1}.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gurmag Angler");
    let zombie = reg.interner_mut().intern("Zombie");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
