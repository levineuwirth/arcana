//! Gorehorn Minotaurs — `{2}{R}{R}` 3/3 Creature — Minotaur Warrior
//! with Bloodthirst 2.
//!
//! Magic 2012 (2011). If an opponent was dealt damage this turn when this
//! creature enters, it gets two additional +1/+1 counters, making it 5/5.
//!
//! # Rules references
//!
//! * CR 702.54 — Bloodthirst. If any player dealt damage to an opponent
//!   this turn before this creature entered, it enters with N +1/+1 counters
//!   (N=2 here). Engine wiring handles the ETB conditional counter placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gorehorn Minotaurs");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Bloodthirst(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
