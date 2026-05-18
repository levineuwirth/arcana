//! Stalker Hag — `{B/G}{B/G}{B/G}` 3/2 Hag with Swampwalk and Forestwalk.
//! Eventide common; a hybrid black-green creature that can't be blocked
//! when the defending player controls a Swamp or a Forest.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. Creature can't be blocked if defending player
//!   controls a land of the specified subtype.
//!   Swampwalk → Landwalk("Swamp"); Forestwalk → Landwalk("Forest").
//!
//! Colors: B, G (hybrid cost — multicolor black+green per spec).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stalker Hag");
    let hag = reg.interner_mut().intern("Hag");
    let swamp = reg.interner_mut().intern("Swamp");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/G}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Landwalk(swamp),
            KeywordAbility::Landwalk(forest),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
