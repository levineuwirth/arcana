//! Phyrexian Marauder — `{X}` 0/0 Artifact Creature — Phyrexian Construct.
//! "This creature enters with X +1/+1 counters on it.
//!  This creature can't block.
//!  This creature can't attack unless you pay {1} for each +1/+1 counter on it."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Marauder");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enters with X +1/+1 counters on it" — enters-with-counters keyed on
    // the spell's X is a static enters replacement with no accessor to X here.
    // GAP: static "can't block" — no continuous can't-block self-static.
    // GAP: "can't attack unless you pay {1} for each +1/+1 counter" — a
    // dynamic attack-tax static is not expressible in this card class.

    reg.register(CardDefinition::new(name, chars))
}
