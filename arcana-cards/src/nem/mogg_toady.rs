//! Mogg Toady — `{1}{R}` 2/2 Goblin.
//! This creature can't attack unless you control more creatures than defending player.
//! This creature can't block unless you control more creatures than attacking player.
//!
//! Both lines are conditional combat-restriction statics with no trigger or
//! cost; they are not expressible as triggered/activated abilities and there
//! is no static-restriction primitive for this comparison — GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mogg Toady");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
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
    // GAP: "can't attack unless you control more creatures than defending
    // player" — conditional static attack restriction, no primitive.
    // GAP: "can't block unless you control more creatures than attacking
    // player" — conditional static block restriction, no primitive.
    reg.register(CardDefinition::new(name, chars))
}
