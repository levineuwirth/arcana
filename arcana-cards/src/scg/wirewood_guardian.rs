//! Wirewood Guardian — `{5}{G}{G}` 6/6 Elf Mutant with Forestcycling {2}
//! (Onslaught, common).
//!
//! # Rules text
//!
//! Forestcycling {2} ({2}, Discard this card: Search your library for a
//! Forest card, reveal it, put it into your hand, then shuffle.)
//!
//! # Keyword mapping notes
//!
//! Scryfall parses this card as: Landcycling, Forestcycling, Typecycling,
//! Cycling. Per engine conventions, all typecycling/landcycling variants
//! collapse to the generic `Cycling` with their printed cost. Emitting a
//! single `KeywordAbility::Cycling(ManaCost::parse("{2}"))`.
//!
//! # Rules references
//!
//! * CR 702.28 — Cycling. A player may pay the cycling cost and discard the
//!   card to search their library for the appropriate land type. The
//!   type-search variant is not separately modeled in the engine; the
//!   `Cycling` keyword records the cost.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wirewood Guardian");
    let elf = reg.interner_mut().intern("Elf");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
