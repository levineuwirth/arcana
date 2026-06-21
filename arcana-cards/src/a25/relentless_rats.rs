//! Relentless Rats — `{1}{B}{B}` 2/2 Rat.
//!
//! Oracle:
//! * This creature gets +1/+1 for each other creature on the battlefield
//!   named Relentless Rats.
//! * A deck can have any number of cards named Relentless Rats.
//!
//! Both lines are static. The dynamic +1/+1-per-other-Relentless-Rats P/T
//! modifier has no primitive and is GAP'd; the deck-construction allowance is
//! a rule with no engine representation and is GAP'd. Only the bones survive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Relentless Rats");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);

    // GAP: static — "gets +1/+1 for each other creature named Relentless Rats"
    // (dynamic P/T modifier static).
    // GAP: rule — "A deck can have any number of cards named Relentless Rats"
    // (deck-construction allowance; no engine representation).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
