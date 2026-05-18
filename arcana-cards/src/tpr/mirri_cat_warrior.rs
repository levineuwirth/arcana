//! Mirri, Cat Warrior — `{1}{G}{G}` 2/3 Legendary Cat Warrior with
//! First Strike, Forestwalk, and Vigilance.
//! Exodus rare; Mirri is a companion of Gerrard and a powerful
//! combat threat that combines three keywords for maximum utility.
//!
//! # Rules references
//!
//! * CR 702.7 — First Strike. Mirri deals combat damage before
//!   creatures without first strike.
//! * CR 702.14 — Landwalk (Forest subtype). This creature can't be
//!   blocked as long as defending player controls a Forest.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Forest"`.
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature
//!   to tap.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirri, Cat Warrior");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);

    let forest = reg.interner_mut().intern("Forest");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Landwalk(forest),
            KeywordAbility::Vigilance,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
