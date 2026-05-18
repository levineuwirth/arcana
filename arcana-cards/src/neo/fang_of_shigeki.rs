//! Fang of Shigeki — `{G}` 1/1 Enchantment Creature — Snake Ninja with
//! Deathtouch. A green enchantment creature from Kamigawa: Neon Dynasty.
//!
//! # Rules references
//!
//! * CR 702.2 — Deathtouch. Any amount of damage this creature deals to
//!   another creature is enough to destroy it.
//!
//! This card is both an Enchantment and a Creature; both type flags are set.
//! Deathtouch is a base characteristic; the runtime pipeline handles enforcement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fang of Shigeki");
    let snake = reg.interner_mut().intern("Snake");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: (TypeLine::ENCHANTMENT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
