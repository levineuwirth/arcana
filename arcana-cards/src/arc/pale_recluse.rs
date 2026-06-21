//! Pale Recluse — `{4}{G}{W}` 4/5 Spider with Reach.
//! "Reach.
//!  Forestcycling {2}, plainscycling {2} ({2}, Discard this card: Search
//!  your library for a Forest or Plains card, reveal it, put it into your
//!  hand, then shuffle.)"
//!
//! Reach is a base keyword. The landcycling abilities are modeled with the
//! generic `Cycling` keyword at their printed {2} cost (per the engine
//! convention that typecycling/landcycling variants emit generic Cycling;
//! the type-search variant is not separately modeled, so the synthesized
//! ability draws a card rather than tutoring a basic land type).
//! GAP: the land-type search payoff of forestcycling/plainscycling is
//! approximated by generic Cycling {2}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pale Recluse");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
