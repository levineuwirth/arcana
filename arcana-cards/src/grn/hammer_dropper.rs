//! Hammer Dropper — `{2}{R}{W}` 5/2 Giant Soldier with Mentor.
//! Guilds of Ravnica common; a four-mana Boros creature with high power
//! that places +1/+1 counters on weaker attackers via Mentor.
//!
//! # Rules references
//!
//! * CR 702.134 — Mentor. Whenever this creature attacks, put a +1/+1
//!   counter on target attacking creature with lesser power. Engine
//!   wiring lives in the combat attack trigger pipeline.
//!
//! Mentor is a fully-implemented keyword; listing it in `keywords`
//! is sufficient — the runtime pipeline handles the trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hammer Dropper");
    let giant = reg.interner_mut().intern("Giant");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Mentor],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
