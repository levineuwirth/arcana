//! Faerie Squadron — `{U}` 1/1 Faerie.
//! Kicker {3}{U}. If kicked, it enters with two +1/+1 counters and with flying.
//!
//! Kicker is not in the usable keyword surface, and a kicker-gated enters-with rider
//! is not expressible — GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Faerie Squadron");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Kicker {3}{U} — keyword not in the usable surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "if this creature was kicked, it enters with two +1/+1 counters and with flying" —
    //      kicker-gated enters-with rider is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
