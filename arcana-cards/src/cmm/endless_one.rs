//! Endless One — `{X}` 0/0 colorless Eldrazi.
//! "This creature enters with X +1/+1 counters on it."
//!
//! Modelled with [`EntersWithSpec::CountersFromX`] (CR 121.6a — the
//! counters are placed as a replacement on entry, so the base 0/0
//! never dies to SBA before receiving them). This mirrors Walking
//! Ballista's X-in-counters wiring.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Endless One");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_enters_with(EntersWithSpec::CountersFromX {
            kind: CounterKind::PlusOnePlusOne,
        }),
    )
}
