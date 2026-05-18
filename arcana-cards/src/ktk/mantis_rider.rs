//! Mantis Rider — `{U}{R}{W}` 3/3 Human Monk with Flying, Vigilance,
//! and Haste.
//!
//! # Rules references
//!
//! * CR 702.9  — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.20 — Vigilance. Attacking doesn't cause the creature to
//!   tap.
//! * CR 702.10 — Haste. This creature can attack and tap as soon as
//!   it comes under its controller's control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mantis Rider");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
