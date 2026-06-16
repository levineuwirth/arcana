//! Formation Breaker — `{1}{G}` 2/1 Beast.
//!
//! Oracle text:
//! * "Creatures with power less than this creature's power can't block
//!   it." — a pure static blocking restriction, not expressible. GAP'd.
//! * "As long as you control a creature with a counter on it, this
//!   creature gets +1/+2." — a pure static conditional P/T buff, not
//!   expressible in this shape. GAP'd.
//!
//! Bones-only registering file: no keywords, two GAP'd statics.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Formation Breaker");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static — "Creatures with power less than this creature's
    // power can't block it."
    // GAP: static — "As long as you control a creature with a counter
    // on it, this creature gets +1/+2."
    reg.register(CardDefinition::new(name, chars))
}
