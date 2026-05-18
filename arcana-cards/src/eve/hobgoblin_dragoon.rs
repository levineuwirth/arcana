//! Hobgoblin Dragoon — `{2}{R/W}` 1/2 red-white Goblin Knight with Flying
//! and First Strike. Hybrid mana cost; colors are both R and W per spec.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.7 — First strike. This creature deals combat damage before
//!   creatures without first strike.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hobgoblin Dragoon");
    let goblin = reg.interner_mut().intern("Goblin");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
