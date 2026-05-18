//! Slitherhead — `{B/G}` 1/1 Plant Zombie with Scavenge {0}.
//! Return to Ravnica common (reprinted in Ravnica Remastered); a
//! hybrid black-green creature that scavenges from the graveyard for
//! free to place +1/+1 counters equal to its power on a creature.
//!
//! # Rules references
//!
//! * CR 702.41a — Scavenge. Activated ability usable only from the
//!   graveyard, only as a sorcery: pay the scavenge cost ({0} here —
//!   Slitherhead scavenges for free), exile this card from your
//!   graveyard, and put a number of +1/+1 counters equal to this
//!   card's power on target creature.
//!   Engine implementation: `KeywordAbility::Scavenge(ManaCost)` —
//!   the registry synthesizes the canonical graveyard activated
//!   ability from the keyword (Pass 4.4b).

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
        keywords: vec![KeywordAbility::Scavenge(
            ManaCost::parse("{0}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
