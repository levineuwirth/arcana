//! Brazen Scourge — `{1}{R}{R}` 3/3 Gremlin with Haste.
//! Uncommon from Kaladesh (2016); an aggressive red three-drop that
//! can attack immediately upon entering the battlefield.
//!
//! # Rules references
//!
//! * CR 702.10 — Haste. The creature can attack and use tap abilities
//!   the turn it enters the battlefield under your control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brazen Scourge");
    let gremlin = reg.interner_mut().intern("Gremlin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gremlin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
