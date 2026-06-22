//! Prehistoric Turtlesaurus — `{4}{G}` 4/5 Creature — Mutant Ninja Turtle
//! Dinosaur. Vigilance.
//! "This spell costs {1} less to cast if you control a creature with a +1/+1
//! counter on it."
//!
//! Vigilance is a base keyword. The conditional cost-reduction static has no
//! expressible primitive in this card class, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prehistoric Turtlesaurus");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);
    subtypes.0.insert(dinosaur);

    // GAP: "This spell costs {1} less to cast if you control a creature with a
    // +1/+1 counter on it" — conditional static cost reduction; not expressible
    // for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
