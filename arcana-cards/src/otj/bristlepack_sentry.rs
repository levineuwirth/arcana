//! Bristlepack Sentry — `{1}{G}` 3/3 Creature — Plant Wolf.
//! Defender.
//! As long as you control a creature with power 4 or greater, this creature can
//! attack as though it didn't have defender.
//!
//! Decomposition:
//! 1. Keyword line: Defender.
//! 2. The second line is a PURE STATIC ability (conditional "can attack as
//!    though it didn't have defender"). No trigger word, no activation cost,
//!    and no Effect that conditionally lifts Defender — GAP. Only the keyword
//!    and bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bristlepack Sentry");
    let plant = reg.interner_mut().intern("Plant");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(wolf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
