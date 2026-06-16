//! Lurking Automaton — `{5}` 0/0 Artifact Creature — Construct.
//! "Reveal this card as you draft it and note how many cards you've drafted
//!  this draft round, including this card. This creature enters with X +1/+1
//!  counters on it, where X is the highest number you noted for cards named
//!  Lurking Automaton."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lurking Automaton");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: draft-round note mechanic + "enters with X +1/+1 counters where X is
    // the highest number you noted" — draft-time state is not modeled by the engine.
    reg.register(CardDefinition::new(name, chars))
}
