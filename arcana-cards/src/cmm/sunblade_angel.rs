//! Sunblade Angel — `{5}{W}` 3/3 Angel with Flying, First strike,
//! Vigilance, and Lifelink. A four-keyword white Angel.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying.
//! * CR 702.7 — First Strike.
//! * CR 702.20 — Vigilance.
//! * CR 702.15 — Lifelink. Damage dealt by this creature also causes its
//!   controller to gain that much life.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunblade Angel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
