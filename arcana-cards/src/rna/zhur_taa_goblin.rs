//! Zhur-Taa Goblin — `{R}{G}` 2/2 Goblin Berserker with Riot.
//! Ravnica Allegiance common; a Gruul Riot creature that enters with
//! either a +1/+1 counter or haste, at the controller's choice.
//!
//! # Rules references
//!
//! * CR 702.133 — Riot. As this creature enters, choose one: put a +1/+1
//!   counter on it, or it gains haste until end of turn. The choice and
//!   its effects are handled by the Riot keyword wiring in the engine.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zhur-Taa Goblin");
    let goblin = reg.interner_mut().intern("Goblin");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Riot],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
