//! Blightbeetle — `{1}{B}` 1/1 black Insect.
//!
//! * "Protection from green" — Protection is not in the usable keyword
//!   surface, GAP'd.
//! * "Creatures your opponents control can't have +1/+1 counters put on
//!   them." A static counter-prohibition; no demonstrated effect or
//!   replacement expresses "can't have counters put on", GAP'd.
//!
//! No abilities are expressible with the demonstrated API; only the
//! bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blightbeetle");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    // GAP: Protection from green — Protection keyword not usable here.
    // GAP: static "creatures your opponents control can't have +1/+1
    // counters put on them" — no counter-prohibition effect available.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
