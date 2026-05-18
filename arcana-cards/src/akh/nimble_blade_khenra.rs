//! Nimble-Blade Khenra — `{1}{R}` 1/3 Jackal Warrior.
//! Has Prowess (keyword not yet in engine API; best-effort stub).
//!
//! # Rules references
//!
//! * Prowess — Whenever you cast a noncreature spell, this creature gets
//!   +1/+1 until end of turn. Not expressible with current KeywordAbility
//!   variants; verify pipeline will flag for human routing.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nimble-Blade Khenra");
    let jackal = reg.interner_mut().intern("Jackal");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jackal);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
