//! Rust-Shield Rampager — `{3}{G}` 4/4 Creature — Raccoon Warrior.
//!
//! * Offspring {2} — not in the usable keyword surface; GAP'd.
//! * "This creature can't be blocked by creatures with power 2 or
//!   less." — a static conditional-evasion ability with no
//!   triggered/activated hook; GAP'd.
//! Emitted as faithful bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rust-Shield Rampager");
    let raccoon = reg.interner_mut().intern("Raccoon");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(raccoon);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: Offspring {2} (not in usable keyword surface); static
    // "can't be blocked by creatures with power 2 or less" (no
    // triggered/activated hook for a conditional static evasion).

    reg.register(CardDefinition::new(name, chars))
}
