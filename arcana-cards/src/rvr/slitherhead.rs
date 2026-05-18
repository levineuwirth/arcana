//! Slitherhead — `{B/G}` 1/1 Plant Zombie with Scavenge.
//! Ravnica: City of Guilds uncommon; a hybrid black-green creature
//! that can be scavenged from the graveyard for free to place a
//! +1/+1 counter on a target creature.
//!
//! # Rules references
//!
//! * CR 702.96 — Scavenge. Activated ability usable only from the
//!   graveyard; exile this card and put +1/+1 counters equal to its
//!   power on target creature. Scavenge only as a sorcery.
//!   Engine implementation: `KeywordAbility::Scavenge`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slitherhead");
    let plant = reg.interner_mut().intern("Plant");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Scavenge],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
