//! Timber Wolves — `{G}` 1/1 Wolf with Banding.
//! Alpha uncommon (1993); the iconic mono-green banding creature and
//! an early demonstration that Banding is not exclusively a white keyword.
//!
//! # Rules references
//!
//! * CR 702.21 — Banding. Any creatures with banding, and up to one
//!   without, can attack in a band. Bands are blocked as a group. If
//!   any creatures with banding you control are blocking or being
//!   blocked by a creature, you divide that creature's combat damage,
//!   not its controller, among any of the creatures it's being blocked
//!   by or is blocking.
//!
//! Banding is a base characteristic on this card; listing it in
//! `keywords` is sufficient — the runtime pipeline handles the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Timber Wolves");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
