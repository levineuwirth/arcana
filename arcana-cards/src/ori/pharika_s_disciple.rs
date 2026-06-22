//! Pharika's Disciple — `{3}{G}` 2/3 Centaur Warrior with Deathtouch
//! and Renown 1.
//!
//! Both abilities are base keywords:
//! * Deathtouch (evergreen unit keyword).
//! * Renown 1 (parametrized keyword — the engine synthesizes the
//!   "deals combat damage to a player, if not renowned, put a +1/+1
//!   counter and become renowned" trigger).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pharika's Disciple");
    let centaur = reg.interner_mut().intern("Centaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Renown(1)],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
