//! Bloodrage Vampire — `{2}{B}` 3/1 Vampire with Bloodthirst 1.
//! Magic 2012 common; enters with a +1/+1 counter if an opponent was
//! dealt damage this turn.
//!
//! # Rules references
//!
//! * CR 702.54 — Bloodthirst N. If an opponent was dealt damage this
//!   turn, this creature enters the battlefield with N +1/+1 counters
//!   on it. Engine wiring handles the ETB counter placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodrage Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bloodthirst(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
