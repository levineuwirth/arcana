//! Sunweb — `{3}{W}` 5/6 Wall with Flying and Defender.
//!
//! Oracle text:
//! * Defender — keyword.
//! * Flying — keyword.
//! * "This creature can't block creatures with power 2 or less." — a pure
//!   static blocking restriction. There is no `Effect` / continuous-ability
//!   primitive for "can't block creatures with power N or less" in the
//!   demonstrated API, so this static is GAP'd. The two evergreen keywords
//!   are emitted faithfully.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunweb");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static "This creature can't block creatures with power 2 or less"
    // — no demonstrated Effect/continuous primitive for a power-gated
    // can't-block restriction.
    reg.register(CardDefinition::new(name, chars))
}
