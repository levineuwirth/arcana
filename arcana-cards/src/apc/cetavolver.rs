//! Cetavolver — `{1}{U}` 1/1 blue Volver.
//!
//! Rules text:
//! * Kicker {1}{R} and/or {G}. (Kicker is not in the usable KeywordAbility
//!   surface — emitted as `keywords: vec![]` and GAP'd.)
//! * If this creature was kicked with its {1}{R} kicker, it enters with two
//!   +1/+1 counters on it and with first strike. (kicker-conditional ETB rider
//!   — GAP)
//! * If this creature was kicked with its {G} kicker, it enters with a +1/+1
//!   counter on it and with trample. (kicker-conditional ETB rider — GAP)
//!
//! There is no Kicker cost field nor a "was kicked with X" condition in the
//! demonstrated API, so both as-it-enters riders (which gate entirely on which
//! kicker was paid) are unexpressible and GAP'd. No ability is expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cetavolver");
    let volver = reg.interner_mut().intern("Volver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(volver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Kicker is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "If kicked with {1}{R}, enters with two +1/+1 counters and first
    //       strike." — kicker-conditional ETB rider; no Kicker cost / "was
    //       kicked" condition in the API.
    // GAP: "If kicked with {G}, enters with a +1/+1 counter and trample." —
    //       same kicker-conditional ETB rider limitation.

    reg.register(CardDefinition::new(name, chars))
}
