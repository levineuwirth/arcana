//! Goblin War Buggy — `{1}{R}` 2/2 red Goblin with Haste.
//!
//! Oracle text:
//! * "Haste"
//! * "Echo {1}{R}" — upkeep sacrifice-unless-pay.
//!
//! Implementation notes:
//! * Haste is a base keyword characteristic.
//! * Echo is not in the usable keyword surface and has no `TriggerCondition`
//!   / `ActivationCost` shape to model the sacrifice-unless-pay gate.
//!   // GAP: keyword — Echo {1}{R} (upkeep sac-unless-pay) not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin War Buggy");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
