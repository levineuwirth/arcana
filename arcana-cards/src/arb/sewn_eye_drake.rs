//! Sewn-Eye Drake — `{2}{U/R}{B}` 3/1 Zombie Drake with Flying and Haste.
//! Shadowmoor common; a three-color (blue-red-black) creature with a
//! hybrid mana symbol. Has evasion via Flying and immediate-attack
//! capability via Haste.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.10 — Haste. The creature can attack and use {T} abilities
//!   the turn it comes under the controller's control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sewn-Eye Drake");
    let zombie = reg.interner_mut().intern("Zombie");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U/R}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
