//! Utvara Scalper — `{1}{R}` 1/2 red Creature — Goblin Scout.
//!
//! Flying
//! This creature attacks each combat if able.
//!
//! Decomposition: Flying → `keywords`. The "attacks each combat if able"
//! line is a pure static attack-requirement continuous ability with no
//! trigger word and no cost — there is no demonstrated API to express an
//! attack requirement, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Utvara Scalper");
    let goblin = reg.interner_mut().intern("Goblin");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: static "This creature attacks each combat if able." — an
    // attack-requirement continuous ability with no trigger/cost; no
    // demonstrated Effect/Keyword expresses a must-attack restriction.
    reg.register(CardDefinition::new(name, chars))
}
