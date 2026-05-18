//! War Elephant — `{3}{W}` 2/2 Elephant with Trample and Banding.
//! Common from Arabian Nights (1993); one of the original banding
//! creatures. Banding is not expressible via the demonstrated
//! `KeywordAbility` API, so only Trample is registered here; the
//! verify pipeline will flag the Banding gap for human routing.
//!
//! # Rules references
//!
//! * CR 702.6 — Trample. If the creature would assign damage to a
//!   blocker, excess damage may be assigned to the defending player
//!   or planeswalker.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("War Elephant");
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
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
