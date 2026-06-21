//! Winding Constrictor — `{B}{G}` 2/3 Creature — Snake.
//!
//! Oracle:
//! * If one or more counters would be put on an artifact or creature you
//!   control, that many plus one of each of those kinds are put instead.
//! * If you would get one or more counters, you get that many plus one of each
//!   of those kinds instead.
//!
//! Both lines are static counter-replacement effects (CR 614) with no trigger
//! and no cost; there is no replacement-effect hook on this card class, so both
//! are GAP'd and only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Winding Constrictor");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "counters put on an artifact/creature you control get +1" — static
    // counter-replacement effect; no replacement hook on this card class.
    // GAP: "counters you get get +1" — same, for player counters.

    reg.register(CardDefinition::new(name, chars))
}
