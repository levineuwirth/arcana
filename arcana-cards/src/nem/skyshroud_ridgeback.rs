//! Skyshroud Ridgeback — `{G}` 2/3 Beast with Fading 2.
//! Nemesis common; a green beast that enters with two fade counters
//! and is sacrificed when no fade counters remain at the beginning
//! of its controller's upkeep.
//!
//! # Rules references
//!
//! * CR 702.32 — Fading N. This permanent enters with N fade
//!   counters. At the beginning of its controller's upkeep, remove
//!   a fade counter from it. If you can't, sacrifice it. N=2 as
//!   given in the oracle text.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyshroud Ridgeback");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Fading(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
