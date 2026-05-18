//! Hillcomber Giant — `{2}{W}{W}` 3/3 Giant Scout with Mountainwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. Mountainwalk: this creature can't be blocked
//!   as long as the defending player controls a Mountain. Engine wiring
//!   maps Mountainwalk to `KeywordAbility::Landwalk` keyed on the
//!   interned subtype "Mountain".
//!
//! The generic Scryfall `Landwalk` umbrella keyword is ignored; only
//! the specific `Mountainwalk` entry is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hillcomber Giant");
    let giant = reg.interner_mut().intern("Giant");
    let scout = reg.interner_mut().intern("Scout");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
