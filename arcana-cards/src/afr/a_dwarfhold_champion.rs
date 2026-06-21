//! A-Dwarfhold Champion — `{1}{W}` 3/1 Creature — Dwarf Warrior.
//!
//! Oracle:
//! * "Ward {1}" — a mana ward, expressible as `KeywordAbility::Ward`.
//! * "As long as Dwarfhold Champion is equipped, it gets +0/+2." — a
//!   static continuous ability gated on being equipped. GAP: no
//!   demonstrated primitive expresses an equipped-conditional static
//!   pump (not a triggered/activated ability).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Dwarfhold Champion");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: static "as long as ~ is equipped, it gets +0/+2" (equipped-
    // conditional continuous pump) is not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
