//! Boros Swiftblade — `{R}{W}` 1/2 Human Soldier with Double Strike.
//! Ravnica: City of Guilds uncommon (2005); the defining Boros guild
//! creature, representing a swift warrior who deals damage both before
//! and at the same time as defenders, making it punch well above its
//! mana cost when pumped.
//!
//! # Rules references
//!
//! * CR 702.4 — Double Strike. In combat, creatures with double strike
//!   deal damage in both the first combat damage step (like first
//!   strike) and the regular combat damage step. Engine wiring checks
//!   this keyword to schedule two damage assignments.
//!
//! Double strike is a base characteristic on this card, so nothing
//! beyond listing it in `keywords` is required — the runtime
//! pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boros Swiftblade");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
