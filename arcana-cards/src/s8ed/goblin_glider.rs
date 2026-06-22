//! Goblin Glider — `{1}{R}` 1/1 Goblin with Flying.
//! "This creature can't block."
//!
//! Flying is a base keyword. "This creature can't block" is a pure
//! static continuous ability with no trigger/activation hook and no
//! expressible self-static primitive (Effect::ForbidBlocking is a
//! targeted, duration-bound effect, not a permanent self-static), so
//! it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Glider");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: "This creature can't block" — a pure self-static with no
        // expressible permanent primitive.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
