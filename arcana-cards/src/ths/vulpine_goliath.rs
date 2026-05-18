//! Vulpine Goliath — `{4}{G}{G}` 6/5 Fox with Trample.
//! A large green fox that tramples over blockers, dealing
//! excess combat damage to the defending player or planeswalker.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. Excess combat damage can be assigned to the
//!   player or planeswalker the creature is attacking.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vulpine Goliath");
    let fox = reg.interner_mut().intern("Fox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
