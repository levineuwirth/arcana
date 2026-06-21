//! Okk — `{1}{R}` 4/4 Goblin.
//!
//! * This creature can't attack unless a creature with greater power
//!   also attacks.
//! * This creature can't block unless a creature with greater power
//!   also blocks.
//!
//! Both lines are static combat restrictions with no expressible
//! primitive (no "can't attack/block unless …" combat gate). Both are
//! GAP'd; the card carries only its bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Okk");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "can't attack unless a creature with greater power
    // also attacks" — no expressible combat-restriction primitive.
    // GAP: static "can't block unless a creature with greater power also
    // blocks" — same.

    reg.register(CardDefinition::new(name, chars))
}
