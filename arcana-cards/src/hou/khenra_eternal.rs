//! Khenra Eternal — `{1}{B}` 2/2 Zombie Jackal Warrior.
//! Has Afflict 1 (keyword not yet in engine API; best-effort stub).
//!
//! # Rules references
//!
//! * Afflict N — Whenever this creature becomes blocked, defending player
//!   loses N life. Not expressible with current KeywordAbility variants;
//!   verify pipeline will flag for human routing.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Khenra Eternal");
    let zombie = reg.interner_mut().intern("Zombie");
    let jackal = reg.interner_mut().intern("Jackal");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(jackal);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
