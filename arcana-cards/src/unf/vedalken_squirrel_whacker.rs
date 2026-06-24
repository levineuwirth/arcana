//! Vedalken Squirrel-Whacker — `{3}{U}` */* Vedalken Guest.
//!
//! Oracle (both lines set base P/T from six-sided-die ROLLS — the
//! self-CDA engine resolves in-game counts/scalars, but a d6 roll is RNG
//! with no game-state count or scalar to read, so both stay GAP'd and the
//! P/T are left as `*`):
//! * GAP: As this creature enters, roll a d6 twice; its base power
//!   becomes the first result and its base toughness the second.
//! * GAP: If you would roll one or more d6, instead roll them and you
//!   may exchange one result with this creature's base power or base
//!   toughness.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vedalken Squirrel-Whacker");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let guest = reg.interner_mut().intern("Guest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(guest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
