//! Feiyi Snake — `{1}{G}` 2/1 Snake with Reach.
//! Portal Three Kingdoms common; a ground creature capable of
//! blocking flying attackers thanks to Reach, despite having
//! no flying of its own.
//!
//! # Rules references
//!
//! * CR 702.17 — Reach. "A creature with reach can block creatures
//!   with flying." Engine wiring lives in the combat blocker filter
//!   (CR 509.1b), which accepts a non-flying blocker against a
//!   flying attacker iff the blocker has Reach.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feiyi Snake");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
