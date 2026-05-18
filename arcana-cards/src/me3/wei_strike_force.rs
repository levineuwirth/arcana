//! Wei Strike Force — `{2}{B}` 2/1 Human Soldier with Horsemanship.
//! Portal: Three Kingdoms common; a black creature that can only
//! be blocked by creatures that also have horsemanship.
//!
//! # Rules references
//!
//! * CR 702.31 — Horsemanship. This creature can't be blocked except
//!   by creatures with horsemanship. Engine wiring lives in the
//!   combat blocker filter.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wei Strike Force");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Horsemanship],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
