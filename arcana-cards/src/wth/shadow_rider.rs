//! Shadow Rider — `{2}{B}{B}` 3/3 Knight with Flanking.
//! A black flanking knight that punishes blockers without flanking
//! by giving them -1/-1 until end of turn.
//!
//! # Rules references
//!
//! * CR 702.24 — Flanking. Whenever a creature without flanking
//!   blocks this creature, the blocking creature gets -1/-1 until
//!   end of turn. Engine wiring lives in the combat damage pipeline.
//!
//! Flanking is a fully-implemented keyword; listing it in `keywords`
//! is sufficient — the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shadow Rider");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flanking],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
