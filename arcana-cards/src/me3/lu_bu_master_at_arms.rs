//! Lu Bu, Master-at-Arms — `{5}{R}` 4/3 Legendary Human Soldier Warrior
//! with Haste and Horsemanship.
//! Portal Three Kingdoms rare; one of the iconic horsemanship legends
//! from the Three Kingdoms block, Lu Bu combines speed with the
//! Portal-exclusive evasion mechanic.
//!
//! # Rules references
//!
//! * CR 702.10 — Haste. This creature can attack and tap the turn it
//!   enters the battlefield.
//! * CR 702.106 — Horsemanship (fully implemented). This creature
//!   can't be blocked except by creatures with horsemanship.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lu Bu, Master-at-Arms");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Horsemanship],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
