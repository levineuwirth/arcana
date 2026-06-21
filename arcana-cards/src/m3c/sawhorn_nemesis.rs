//! Sawhorn Nemesis — `{3}{R}` 2/4 Creature — Dinosaur.
//! "As this creature enters, choose a player. If a source would deal damage to
//! the chosen player or a permanent they control, it deals double that damage
//! instead."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sawhorn Nemesis");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    // GAP: "As this creature enters, choose a player. If a source would deal
    // damage to the chosen player or a permanent they control, it deals double
    // that damage instead." — a chosen-player-scoped damage-doubling
    // replacement effect is not expressible (no damage-doubling Effect and no
    // "as enters, choose a player" + replacement primitive).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
