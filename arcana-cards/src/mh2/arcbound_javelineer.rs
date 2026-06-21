//! Arcbound Javelineer — `{W}` 0/1 Artifact Creature — Soldier with Modular 1.
//!
//! Oracle text:
//! * "{T}, Remove X +1/+1 counters from this creature: It deals X damage to
//!   target attacking or blocking creature." — GAP: the cost removes a
//!   VARIABLE number (X) of +1/+1 counters and deals that same X as damage.
//!   `remove_self_counter` is a FIXED `(CounterKind, count)` and there is no
//!   way to bind the removed count to the damage amount; the activated
//!   ability is omitted.
//! * Modular 1 — base keyword (enters with a +1/+1 counter; the dies→
//!   move-counters half is handled by engine Modular wiring).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Javelineer");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Modular(1)],
        ..Default::default()
    };

    // GAP: "{T}, Remove X +1/+1 counters: deal X damage to target attacking or
    // blocking creature" — variable-X counter-removal cost tied to a variable
    // damage amount is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
