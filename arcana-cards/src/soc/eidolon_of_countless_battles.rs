//! Eidolon of Countless Battles — `{1}{W}{W}` 0/0 Enchantment Creature
//! — Spirit.
//! Bestow {2}{W}{W}.
//! "This creature and enchanted creature each get +1/+1 for each
//! creature you control and +1/+1 for each Aura you control."
//!
//! GAP: "Bestow {2}{W}{W}" — Bestow is not an expressible KeywordAbility
//! variant for this card class.
//! GAP: "This creature and enchanted creature each get +1/+1 for each
//! creature you control and +1/+1 for each Aura you control." — a pure
//! self/host dynamic-P/T static, not a triggered or activated ability,
//! and not expressible here.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eidolon of Countless Battles");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
