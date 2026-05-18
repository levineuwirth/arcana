//! Akrasan Squire — `{W}` 1/1 Human Soldier with Exalted.
//! A Shards of Alara common; Exalted rewards sending a single
//! creature to attack alone by granting it +1/+1 until end of turn.
//!
//! # Rules references
//!
//! * CR 702.82 — Exalted. Whenever a creature you control attacks
//!   alone, that creature gets +1/+1 until end of turn.
//!
//! NOTE: `KeywordAbility::Exalted` is not present in the currently
//! demonstrated API surface. The keyword list is left empty as a
//! best-effort stub; the verify pipeline will flag this gap for a
//! human to wire the Exalted variant when it is added to the engine.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akrasan Squire");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
