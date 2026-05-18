//! Ghor-Clan Savage — `{3}{G}{G}` 2/3 Creature — Centaur Berserker
//! with Bloodthirst 3.
//!
//! Guildpact (2006). If an opponent was dealt damage this turn when this
//! creature enters, it gets three additional +1/+1 counters, making it 5/6.
//!
//! # Rules references
//!
//! * CR 702.54 — Bloodthirst. If any player dealt damage to an opponent
//!   this turn before this creature entered, it enters with N +1/+1 counters
//!   (N=3 here). Engine wiring handles the ETB conditional counter placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghor-Clan Savage");
    let centaur = reg.interner_mut().intern("Centaur");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Bloodthirst(3)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
