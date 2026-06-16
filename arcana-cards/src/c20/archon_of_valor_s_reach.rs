//! Archon of Valor's Reach — `{4}{G}{W}` 5/6 Archon with Flying, Vigilance,
//! and Trample.
//! "As this creature enters, choose artifact, enchantment, instant, sorcery,
//!  or planeswalker. Players can't cast spells of the chosen type."
//!
//! GAP (static): the "choose a card type as this enters, then players can't
//! cast spells of that type" is a choose-on-enter continuous restriction
//! with no expressible Effect / replacement primitive in this surface —
//! omitted. Only the keyword line is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archon of Valor's Reach");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
