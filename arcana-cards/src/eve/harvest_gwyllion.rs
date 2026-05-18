//! Harvest Gwyllion — `{2}{W/B}{W/B}` 2/4 Hag with Wither.
//! Eventide uncommon; a white-black hybrid creature that deals damage
//! to creatures as -1/-1 counters.
//!
//! # Rules references
//!
//! * CR 702.77 — Wither. This creature deals damage to creatures in
//!   the form of -1/-1 counters. Engine wiring substitutes the
//!   standard damage application for counter placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harvest Gwyllion");
    let hag = reg.interner_mut().intern("Hag");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W/B}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
