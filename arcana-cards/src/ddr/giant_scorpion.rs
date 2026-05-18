//! Giant Scorpion — `{2}{B}` 1/3 Scorpion with Deathtouch.
//! A classic black defensive creature; any amount of damage
//! it deals destroys the target creature outright.
//!
//! # Rules references
//!
//! * CR 702.2 — Deathtouch. Any amount of damage this creature deals
//!   to another creature is enough to destroy it.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Scorpion");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
