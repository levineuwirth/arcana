//! Anurid Murkdiver — `{4}{B}{B}` 4/3 Zombie Frog Beast with Swampwalk.
//! Onslaught common; a large black zombie creature that sneaks through
//! Swamp-rich boards unblocked.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Swamp subtype). This creature can't be
//!   blocked as long as defending player controls a Swamp.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Swamp"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anurid Murkdiver");
    let zombie = reg.interner_mut().intern("Zombie");
    let frog = reg.interner_mut().intern("Frog");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(frog);
    subtypes.0.insert(beast);

    let swamp = reg.interner_mut().intern("Swamp");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
