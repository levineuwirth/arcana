//! Spawn of Rix Maadi — `{3}{B}{R}` 5/3 Horror with Unleash.
//! Return to Ravnica uncommon; a large Rakdos Unleash body that trades
//! blocking ability for the option to enter with a +1/+1 counter.
//!
//! # Rules references
//!
//! * CR 702.96 — Unleash. As this creature enters, you may put a +1/+1
//!   counter on it. If it has a +1/+1 counter on it, it can't block.
//!   Both ETB-scaling and the blocking restriction are handled by the
//!   Unleash keyword wiring in the engine.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spawn of Rix Maadi");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
