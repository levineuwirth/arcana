//! Karakyk Guardian — `{3}{G}{U}{R}` 6/5 Dragon.
//! Flying, vigilance, trample.
//! This creature has hexproof if it hasn't dealt damage yet.
//!
//! GAP: "This creature has hexproof if it hasn't dealt damage yet" is a
//! conditional static continuous ability (no trigger word, no cost) — not
//! expressible as a triggered/activated ability. Only the keyword line is
//! emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karakyk Guardian");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
