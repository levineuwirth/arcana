//! Impetuous Sunchaser — `{1}{R}` 1/1 Human Soldier.
//!
//! Oracle:
//! * Flying, haste  (keyword line)
//! * This creature attacks each combat if able.  (GAP — a static
//!   attack-requirement continuous ability with no trigger word and no
//!   activation cost; not expressible with the demonstrated API.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Impetuous Sunchaser");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "This creature attacks each combat if able." — a static attack
    // requirement; no demonstrated API to express a must-attack restriction.

    reg.register(CardDefinition::new(name, chars))
}
