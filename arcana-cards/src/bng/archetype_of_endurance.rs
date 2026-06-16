//! Archetype of Endurance — `{6}{G}{G}` 6/5 Enchantment Creature — Boar.
//! "Creatures you control have hexproof."
//! "Creatures your opponents control lose hexproof and can't have or gain
//! hexproof." (both pure statics — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archetype of Endurance");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "Creatures you control have hexproof" — pure keyword-granting static.
    // GAP: "Creatures your opponents control lose hexproof and can't have or
    // gain hexproof" — pure ability-stripping/restriction static.
    reg.register(CardDefinition::new(name, chars))
}
