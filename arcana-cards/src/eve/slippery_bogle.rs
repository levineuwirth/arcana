//! Slippery Bogle — Eventide common. `{G/U}` 1/1 Elemental Hound
//! with Hexproof. Printed with literally no other text, which is
//! exactly why it's the ideal Hexproof seed: every test against it
//! exercises the keyword in isolation, with no secondary abilities
//! muddying the fixture state.
//!
//! # Rules references
//!
//! * CR 702.11b — Hexproof. "This object can't be the target of
//!   spells or abilities your opponents control." Checked by
//!   [`arcana_core::targets::TargetRequirement::matches_choice`]
//!   via [`arcana_core::state::GameState::has_keyword`] (layer-
//!   aware); the same entry point runs at announce (CR 601.2c) and
//!   at resolution (CR 608.2b), so a creature that gained Hexproof
//!   in response fizzles the targeting spell at resolution.
//!
//! # Engine wiring
//!
//! Hexproof is declarative — no per-card effect function, no
//! activated/triggered ability. The keyword on
//! `base_characteristics.keywords` is all the engine needs; the
//! target-filter pipeline consults it via the layer-aware helper
//! and rejects opponent-controlled targeting at both announce and
//! resolution.
//!
//! # Why hybrid cost
//!
//! Slippery Bogle's printed mana cost is `{G/U}` (hybrid
//! green-blue). The engine's mana solver supports hybrid costs via
//! [`arcana_core::mana::ManaCostComponent::Hybrid`], so the printed
//! cost rides as-is with no simplification — the seed card exercises
//! hybrid payment on the cast path as an incidental benefit.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slippery Bogle");
    let elemental = reg.interner_mut().intern("Elemental");
    let hound = reg.interner_mut().intern("Hound");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(hound);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/U}").expect("valid hybrid cost")),
        // Hybrid cards' color identity includes both halves (CR 202.4).
        colors: ColorSet::green().with(arcana_core::types::Color::Blue),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
