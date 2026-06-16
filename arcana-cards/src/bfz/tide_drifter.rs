//! Tide Drifter — `{1}{U}` 0/5 Creature — Eldrazi Drone.
//! Devoid (this card has no color).
//! "Other colorless creatures you control get +0/+1." — pure static anthem
//! (no triggered/activated hook); GAP.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tide Drifter");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        // Devoid: the card is colorless despite the blue pip in its cost.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    // GAP: "Other colorless creatures you control get +0/+1" — a static
    // continuous anthem with no triggered/activated hook; not expressible with
    // the demonstrated MultiAbilityCreature API.
    reg.register(CardDefinition::new(name, chars))
}
