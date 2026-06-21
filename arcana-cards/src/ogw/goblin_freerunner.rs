//! Goblin Freerunner — `{3}{R}` 3/2 Creature — Goblin Warrior Ally.
//!
//! Oracle:
//! * Surge {1}{R} — alternative cast cost if you or a teammate has cast
//!   another spell this turn. (GAP: Surge is a casting-cost mechanic with
//!   no demonstrated API; only the Menace keyword is modeled.)
//! * Menace.
//!
//! Decomposition: Menace → `keywords`; Surge is GAP'd (alternative-cost
//! mechanic, not a triggered/activated ability).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Freerunner");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Surge {1}{R}" — an alternative casting cost (cast for less if you
    // or a teammate cast another spell this turn). No casting-cost mechanic in
    // the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
