//! Goblin Goon — `{3}{R}` 6/6 Goblin Mutant.
//! * "This creature can't attack unless you control more creatures than
//!   defending player."
//! * "This creature can't block unless you control more creatures than
//!   attacking player."
//!
//! Both lines are static conditional combat restrictions with no
//! expressible primitive — GAP'd. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Goon");
    let goblin = reg.interner_mut().intern("Goblin");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: static "can't attack unless you control more creatures than
    // defending player" and "can't block unless you control more creatures
    // than attacking player" — no expressible conditional-combat-restriction
    // primitive.
    reg.register(CardDefinition::new(name, chars))
}
