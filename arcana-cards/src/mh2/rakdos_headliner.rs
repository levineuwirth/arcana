//! Rakdos Headliner — `{B}{R}` 3/3 Creature — Devil.
//!
//! * Haste.
//! * Echo—Discard a card.
//!
//! Haste is a base keyword. Echo (an upkeep "sacrifice unless you pay the echo
//! cost" mechanic, here a discard cost) has no usable `KeywordAbility` variant
//! and its first-upkeep gating/payment is not expressible with the supported
//! surface, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos Headliner");
    let devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Echo—Discard a card" — Echo is not in the supported keyword
    // surface; its first-upkeep sacrifice-unless-you-pay gating cannot be
    // expressed.
    reg.register(CardDefinition::new(name, chars))
}
