//! Stalwart Aven — `{2}{W}` 1/3 Creature — Bird Soldier.
//!
//! Oracle:
//! * Flying.
//! * Renown 1.
//!
//! Both are keywords (Renown is a parametrized keyword; the engine synthesizes
//! the combat-damage renown trigger from `KeywordAbility::Renown(1)`), so only
//! the `keywords` line is needed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stalwart Aven");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Renown(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
