//! A-Silver-Fur Master — `{U}{B}` 2/2 Creature — Rat Ninja.
//! Ninjutsu {U/B}. (keyword not in usable surface — GAP)
//! Ninjutsu abilities you activate cost {1} less to activate. (static — GAP)
//! Other Ninja and Rogue creatures you control get +1/+1. (static anthem — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Silver-Fur Master");
    let rat = reg.interner_mut().intern("Rat");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ninjutsu {U/B} — keyword not in usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Ninjutsu abilities you activate cost {1} less to activate."
    // GAP: static anthem "Other Ninja and Rogue creatures you control get +1/+1."

    reg.register(CardDefinition::new(name, chars))
}
