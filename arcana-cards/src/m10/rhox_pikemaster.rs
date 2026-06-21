//! Rhox Pikemaster — `{2}{W}{W}` 3/3 Rhino Soldier.
//!
//! "First strike.
//!  Other Soldier creatures you control have first strike."
//!
//! First strike is a base keyword. The "other Soldiers have first strike"
//! clause is a pure static continuous ability (a keyword-granting anthem);
//! there is no triggered/activated wiring for a permanent's own static
//! grant in this card class, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rhox Pikemaster");
    let rhino = reg.interner_mut().intern("Rhino");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: static continuous anthem "Other Soldier creatures you control
    // have first strike" — not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
