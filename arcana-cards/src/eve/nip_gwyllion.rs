//! Nip Gwyllion — `{W/B}` 1/1 Hag with Lifelink. A hybrid white-black
//! creature from Eventide; uses a hybrid mana symbol but is both
//! white and black.
//!
//! # Rules references
//!
//! * CR 702.15 — Lifelink. Damage dealt by this creature also causes its
//!   controller to gain that much life.
//!
//! Lifelink is a base characteristic; the runtime pipeline handles enforcement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nip Gwyllion");
    let hag = reg.interner_mut().intern("Hag");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
