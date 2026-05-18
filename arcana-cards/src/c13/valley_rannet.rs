//! Valley Rannet — `{4}{R}{G}` 6/3 Beast with Mountaincycling {2} and
//! Forestcycling {2} (Onslaught, common).
//!
//! # Rules text
//!
//! Mountaincycling {2}, forestcycling {2} ({2}, Discard this card: Search
//! your library for a Mountain or Forest card, reveal it, put it into your
//! hand, then shuffle.)
//!
//! # Keyword mapping notes
//!
//! Scryfall parses this card as: Mountaincycling, Landcycling, Forestcycling,
//! Typecycling, Cycling. Per engine conventions, all typecycling/landcycling
//! variants collapse to the generic `Cycling` with their printed cost. Both
//! mountaincycling and forestcycling share cost {2}, so a single
//! `KeywordAbility::Cycling(ManaCost::parse("{2}"))` entry is emitted.
//!
//! # Rules references
//!
//! * CR 702.28 — Cycling. A player may pay the cycling cost and discard the
//!   card to draw a card (or search their library for the appropriate land/
//!   creature type in the typecycling variant). The type-search variant is not
//!   separately modeled in the engine; the `Cycling` keyword records the cost.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valley Rannet");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
