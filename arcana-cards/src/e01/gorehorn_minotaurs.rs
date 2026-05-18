//! Gorehorn Minotaurs — `{2}{R}{R}` 3/3 Minotaur Warrior with Bloodthirst 2.
//! Bloodthirst is not representable with the demonstrated KeywordAbility
//! variants; keywords left empty for verify pipeline.
//!
//! # Rules references
//!
//! * CR 702.54 — Bloodthirst N. If an opponent was dealt damage this turn,
//!   this creature enters with N +1/+1 counters. Not expressible with the
//!   demonstrated API.

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
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
