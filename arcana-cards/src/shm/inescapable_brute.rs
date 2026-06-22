//! Inescapable Brute — `{5}{R}` 3/3 Giant Warrior with Wither.
//!
//! Oracle:
//! * Wither (deals damage to creatures as -1/-1 counters).
//! * "This creature must be blocked if able." — a static combat-
//!   restriction (a lure-style "must be blocked") with no triggered or
//!   activated form and no Effect to express it. GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inescapable Brute");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    // GAP: static "This creature must be blocked if able" — a continuous
    // combat-restriction static, not a triggered/activated ability, with
    // no Effect to express it.
    reg.register(CardDefinition::new(name, chars))
}
