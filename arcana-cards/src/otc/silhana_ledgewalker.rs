//! Silhana Ledgewalker — `{1}{G}` 1/1 green Elf Rogue.
//!
//! Oracle:
//! * Hexproof
//! * This creature can't be blocked except by creatures with flying.
//!
//! Hexproof is a base keyword. The conditional block restriction ("can't be
//! blocked except by creatures with flying") is a static evasion ability;
//! the surface only exposes `Effect::CantBeBlocked` (fully unblockable),
//! which would mis-state the card, so the restriction is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silhana Ledgewalker");
    let elf = reg.interner_mut().intern("Elf");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(rogue);

    // GAP: static "This creature can't be blocked except by creatures with
    // flying." — no conditional-evasion (block-except-by-filter) primitive in
    // the demonstrated surface; `CantBeBlocked` would over-state it.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
