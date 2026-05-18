//! Anaconda — `{3}{G}` 3/3 Snake with Swampwalk.
//! Mirage common; a green Snake that becomes unblockable against
//! Swamp-controlling opponents, representing the anaconda's swamp
//! habitat.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Swamp subtype). This creature can't be
//!   blocked as long as defending player controls a Swamp.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Swamp"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anaconda");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let swamp = reg.interner_mut().intern("Swamp");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
