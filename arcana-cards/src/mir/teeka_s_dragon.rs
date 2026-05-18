//! Teeka's Dragon — `{9}` 5/5 colorless Artifact Creature — Dragon
//! with Flying, Trample, and Rampage 4.
//! Mirage rare; a colorless artifact dragon that grows larger for
//! each creature blocking it beyond the first.
//!
//! # Rules references
//!
//! * CR 702.9  — Flying. Can only be blocked by creatures with
//!   Flying or Reach. Engine wiring lives in the combat blocker
//!   filter.
//! * CR 702.19 — Trample. Excess combat damage is assigned to the
//!   defending player or planeswalker. Engine wiring lives in
//!   `apply_combat_damage`.
//! * CR 702.23 — Rampage N. Whenever this creature becomes blocked,
//!   it gets +N/+N until end of turn for each creature blocking it
//!   beyond the first. N=4 per oracle text ("rampage 4").
//!
//! All three keywords are fully implemented; listing them in
//! `keywords` is sufficient — the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teeka's Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Rampage(4),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
