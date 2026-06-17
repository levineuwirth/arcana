//! Wayward Angel — `{4}{W}{W}` 4/4 Angel Horror with Flying and Vigilance.
//! Threshold — As long as there are seven or more cards in your graveyard,
//! this creature gets +3/+3, is black, has trample, and has "At the beginning
//! of your upkeep, sacrifice a creature."
//!
//! The Threshold clause is a single graveyard-gated STATIC continuous ability
//! (P/T boost + color change + keyword grant + granted upkeep trigger). It is
//! not a triggered or activated ability and has no expressible primitive here,
//! so it is GAP'd; only the Flying/Vigilance keyword line is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wayward Angel");
    let angel = reg.interner_mut().intern("Angel");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP: Threshold static continuous ability (graveyard-gated +3/+3, becomes
    // black, gains trample, gains an upkeep sacrifice trigger) is not
    // expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
