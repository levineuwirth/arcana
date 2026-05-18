//! Rakdos Cackler — `{B/R}` 1/1 Creature — Devil with Unleash.
//! Return to Ravnica uncommon; may enter with a +1/+1 counter but
//! cannot block while it has a +1/+1 counter on it.
//!
//! # Rules references
//!
//! * CR 702.97 — Unleash. You may have this creature enter with a
//!   +1/+1 counter on it. It can't block as long as it has a +1/+1
//!   counter on it. Engine handles both the optional ETB counter and
//!   the blocking restriction.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos Cackler");
    let devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
