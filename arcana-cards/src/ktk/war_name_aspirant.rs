//! War-Name Aspirant — `{1}{R}` 2/1 red Human Warrior.
//! Raid — This creature enters with a +1/+1 counter on it if you attacked this
//! turn.
//! This creature can't be blocked by creatures with power 1 or less.
//!
//! Both non-bones lines are unexpressible with the demonstrated surface:
//!  - Raid enters-with-counter is a conditional ETB-with-counters (no
//!    enters-with-counters / Raid effect primitive) → GAP'd.
//!  - "can't be blocked by creatures with power 1 or less" is a power-filtered
//!    block restriction; CantBeBlocked is all-or-nothing (no power-bound blocker
//!    filter) → GAP'd.
//! Emitted as the vanilla 2/1 bones so the card lands in the catalog.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "Raid — This creature enters with a +1/+1 counter on it if you attacked
// this turn." — conditional enters-with-counters; no Raid / enters-with-counters
// primitive.
// GAP: "This creature can't be blocked by creatures with power 1 or less." —
// power-filtered block restriction; CantBeBlocked has no blocker-power filter.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("War-Name Aspirant");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
