//! Bedrock Tortoise — `{3}{G}` 0/6 Turtle.
//! During your turn, creatures you control have hexproof.
//! Each creature you control with toughness greater than its power assigns
//! combat damage equal to its toughness rather than its power.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bedrock Tortoise");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: "During your turn, creatures you control have hexproof" is a
    // turn-conditional static keyword grant to other permanents — no expressible
    // variant in this card shape (no trigger / no cost).
    // GAP: "creatures you control with toughness > power assign combat damage
    // equal to toughness" is a static combat-damage-assignment replacement with
    // no expressible variant.
    reg.register(CardDefinition::new(name, chars))
}
