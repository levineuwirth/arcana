//! Tapestry Warden — `{3}{G}` 3/4 Artifact Creature — Robot Soldier.
//!
//! Oracle text:
//! * Vigilance — a base keyword.
//! * "Each creature you control with toughness greater than its power
//!   assigns combat damage equal to its toughness rather than its
//!   power." — pure static combat-replacement, not expressible. GAP'd.
//! * "Each creature you control with toughness greater than its power
//!   stations permanents using its toughness rather than its power." —
//!   pure static (stations is unmodeled), not expressible. GAP'd.
//!
//! Bones-only registering file: keyword line + two GAP'd statics.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tapestry Warden");
    let robot = reg.interner_mut().intern("Robot");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static — "Each creature you control with toughness greater
    // than its power assigns combat damage equal to its toughness
    // rather than its power."
    // GAP: static — "Each creature you control with toughness greater
    // than its power stations permanents using its toughness rather
    // than its power." (stations is unmodeled in the engine)
    reg.register(CardDefinition::new(name, chars))
}
