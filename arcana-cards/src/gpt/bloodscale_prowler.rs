//! Bloodscale Prowler — `{2}{R}` 3/1 Lizard Warrior with Bloodthirst 1.
//! Guildpact common; enters with a +1/+1 counter if an opponent was dealt
//! damage this turn (Bloodthirst 1).
//!
//! # Rules references
//!
//! * CR 702.54 — Bloodthirst. If an opponent was dealt damage this turn,
//!   this creature enters with N +1/+1 counters on it. N=1 for this card.
//!   ETB-scaling is handled by the Bloodthirst(1) keyword wiring in the
//!   engine.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodscale Prowler");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bloodthirst(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
