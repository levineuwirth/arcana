//! Gore-House Chainwalker — `{1}{R}` 2/1 Human Warrior with Unleash.
//! Return to Ravnica common; one of the Rakdos guild's Unleash creatures,
//! trading blocking ability for the option to enter with a +1/+1 counter.
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
    let name = reg.interner_mut().intern("Gore-House Chainwalker");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
