//! Marsh Threader — `{1}{W}` 2/1 Kor Scout with Swampwalk.
//! A white evasion creature that cannot be blocked while the defending
//! player controls a Swamp.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Swampwalk). The creature is unblockable
//!   while the defending player controls a land of the named type.
//!
//! NOTE: Swampwalk is a Landwalk variant. The current API's
//! `KeywordAbility` enum does not expose a Landwalk/Swampwalk variant
//! in the demonstrated examples. The keyword is omitted here; the
//! verify pipeline should route this for manual wiring.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marsh Threader");
    let kor = reg.interner_mut().intern("Kor");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
