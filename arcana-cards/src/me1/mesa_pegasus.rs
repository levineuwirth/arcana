//! Mesa Pegasus — `{1}{W}` 1/1 Pegasus with Flying and Banding.
//! Alpha common (1993); an early white flyer combining aerial evasion
//! with banding's cooperative combat-damage-assignment control.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach. Engine wiring lives in [`arcana_core::combat`]'s
//!   blocker filter.
//! * CR 702.21 — Banding. Any creatures with banding, and up to one
//!   without, can attack in a band. Bands are blocked as a group. If
//!   any creatures with banding you control are blocking or being
//!   blocked by a creature, you divide that creature's combat damage,
//!   not its controller, among any of the creatures it's being blocked
//!   by or is blocking.
//!
//! Both keywords are base characteristics on this card; listing them
//! in `keywords` is sufficient — the runtime pipeline handles the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mesa Pegasus");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
