//! Shrike Force — `{2}{W}` 1/3 Bird Knight with Flying, Double Strike, and Vigilance.
//!
//! # Rules references
//!
//! * CR 702.9  — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.4  — Double Strike. This creature deals both first-strike and
//!   regular combat damage.
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature to tap.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shrike Force");
    let bird = reg.interner_mut().intern("Bird");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::DoubleStrike,
            KeywordAbility::Vigilance,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
