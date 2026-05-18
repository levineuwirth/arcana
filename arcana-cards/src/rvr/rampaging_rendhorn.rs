//! Rampaging Rendhorn — `{4}{G}` 4/4 Creature — Beast with Riot.
//!
//! Ravnica Allegiance (2019). Riot gives a choice on entry: a +1/+1 counter
//! or haste.
//!
//! # Rules references
//!
//! * CR 702.138 — Riot. As the creature enters the battlefield, its
//!   controller chooses to put one +1/+1 counter on it or to give it haste
//!   until end of turn. Engine wiring handles the ETB choice.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rampaging Rendhorn");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Riot],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
