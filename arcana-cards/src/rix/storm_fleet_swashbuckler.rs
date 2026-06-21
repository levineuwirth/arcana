//! Storm Fleet Swashbuckler — `{1}{R}` 2/2 Human Pirate.
//! Ascend.
//! This creature has double strike as long as you have the city's
//! blessing.
//!
//! Ascend / the city's blessing is not a supported keyword or game
//! state, and the double-strike clause is a pure static keyed on it, so
//! neither is expressible. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storm Fleet Swashbuckler");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Ascend / city's blessing not modeled.
    // GAP: "double strike while you have the city's blessing" — pure
    //      static keyed on the unmodeled city's blessing.
    reg.register(CardDefinition::new(name, chars))
}
