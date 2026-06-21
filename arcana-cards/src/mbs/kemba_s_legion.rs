//! Kemba's Legion — `{5}{W}{W}` 4/6 Cat Soldier with Vigilance.
//!
//! Oracle:
//! * Vigilance
//! * This creature can block an additional creature each combat for each
//!   Equipment attached to this creature.
//!
//! GAP: "can block an additional creature each combat for each Equipment
//! attached" is a static combat-rule modification with no `Effect` /
//! triggered / activated form in the demonstrated API (no
//! extra-blocks / multiblock primitive, no per-attached-Equipment count
//! hook). Only the Vigilance keyword is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kemba's Legion");
    let cat = reg.interner_mut().intern("Cat");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
