//! Wu Light Cavalry — `{1}{U}` 1/2 Human Soldier with Horsemanship.
//! Portal Three Kingdoms common; a cheap blue creature with the
//! Portal-exclusive horsemanship evasion, representing the mobile
//! mounted units of the Kingdom of Wu.
//!
//! # Rules references
//!
//! * CR 702.106 — Horsemanship (fully implemented). This creature
//!   can't be blocked except by creatures with horsemanship.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wu Light Cavalry");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Horsemanship],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
