//! Hypersonic Dragon — `{3}{U}{R}` 4/4 Dragon.
//! Flying, haste.
//! "You may cast sorcery spells as though they had flash."
//!
//! Keyword line → `keywords: vec![Flying, Haste]`. The flash-granting
//! static permission has no expressible primitive in this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hypersonic Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP (static): "You may cast sorcery spells as though they had flash."
    // No primitive grants alternate-timing casting permission in this class.
    reg.register(CardDefinition::new(name, chars))
}
