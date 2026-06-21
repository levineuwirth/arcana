//! Skymarcher Aspirant — `{W}` 2/1 white Vampire Soldier.
//!
//! Oracle:
//! * Ascend (GAP — not in the usable keyword surface; the city's
//!   blessing mechanic is not modeled here)
//! * "This creature has flying as long as you have the city's
//!   blessing." (conditional static keyword grant — GAP)
//!
//! Neither ability is expressible with the demonstrated API: Ascend
//! has no usable keyword variant, and the city's-blessing-gated Flying
//! is a conditional continuous static. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skymarcher Aspirant");
    let vampire = reg.interner_mut().intern("Vampire");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(soldier);

    // GAP: Ascend — no usable keyword variant; city's blessing unmodeled.
    // GAP: "has flying as long as you have the city's blessing" —
    // conditional static keyword grant, not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
