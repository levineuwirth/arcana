//! Noble Elephant — `{3}{W}` 2/2 Elephant with Trample and Banding.
//! Mirage common; a straightforward white trample-banding creature in the
//! same vein as War Elephant, emphasising the noble-beast flavour.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. Excess combat damage may be assigned to the
//!   defending player or planeswalker even when the creature is blocked.
//! * CR 702.22 — Banding. Creatures with banding can attack or block as a band,
//!   with the controller of creatures with banding in the band assigning combat
//!   damage for any creature blocked by or blocking the band.
//!
//! Both keywords are base characteristics; listing them in `keywords` is
//! sufficient — the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noble Elephant");
    let elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Banding, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
