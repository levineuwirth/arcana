//! Lightning Angel — `{1}{U}{R}{W}` 3/4 Angel with Flying, Vigilance,
//! and Haste. A three-color (R/U/W) creature from Apocalypse.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature to tap.
//! * CR 702.10 — Haste. This creature can attack and use tap abilities the
//!   turn it enters the battlefield.
//!
//! All three keywords are base characteristics; the runtime pipelines handle
//! enforcement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lightning Angel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
