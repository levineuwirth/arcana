//! Goblin Grappler — `{R}` 1/1 Goblin with Provoke.
//! Onslaught common; when this creature attacks, the defending player
//! may have a target creature they control untap and block it if able.
//!
//! # Rules references
//!
//! * CR 702.38 — Provoke. Whenever this creature attacks, you may
//!   have target creature defending player controls untap and block
//!   this creature if able. Engine wiring lives in the combat
//!   declare-blockers pipeline.
//!
//! Provoke is a fully-implemented keyword; listing it in `keywords`
//! is sufficient — the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Grappler");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Provoke],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
