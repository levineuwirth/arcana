//! Kavu Titan — `{1}{G}` 2/2 Kavu.
//! Kicker {2}{G}; if kicked, enters with three +1/+1 counters and trample.
//! GAP — Kicker is not a usable keyword and kicked-state ETB modification is
//! not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kavu Titan");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker {2}{G} is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "If this creature was kicked, it enters with three +1/+1 counters
    // and trample" — the kicked-state conditional ETB is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
