//! Murktide Regent — `{3}{U}{U}` flying Dragon with delve. Enters
//! with a +1/+1 counter for each card exiled with it (the delve
//! payment). The canonical test for delve-count → ETB P/T.
//!
//! # Rules references
//!
//! * CR 702.66 — Delve. Each card exiled from your graveyard while
//!   paying costs pays for `{1}`. Engine-side, delve exile is tracked
//!   per [`crate::arcana_core::actions::CostReductions`] and the
//!   caster's chosen exiles flow through [`apply_cast_spell`] into
//!   [`StackEntry::delve_count`].
//! * CR 121.6a — "enters with" clauses. Modelled via
//!   [`EntersWithSpec::CountersFromDelveCount`]; counter placement
//!   goes through [`GameState::place_counters`] so Hardened Scales,
//!   Doubling Season, etc. still compose.
//!
//! # Simplification
//!
//! The printed text is "…for each *instant and sorcery* card exiled
//! with it." We currently count **all** cards exiled via delve
//! regardless of type. In practice this matters only when the
//! caster delves a non-instant/sorcery (an artifact, a creature
//! card) — rare in the decks that want Murktide. The gap is
//! documented so a future commit can store `delve_exiles: Vec<ObjectId>`
//! on the stack entry and filter by type at ETB time.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Murktide Regent");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // Base 3/3. Delve-count +1/+1 counters layer on via
        // `EntersWithSpec::CountersFromDelveCount`, so effective P/T
        // grows with the number of cards the caster exiled.
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // Printed keywords: Flying + Delve. Delve is detected by
        // `has_delve` walking `effective_keywords`, so declaring it
        // here is what unlocks the delve cost-reduction pipeline
        // for this card.
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Delve],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromDelveCount {
                kind: CounterKind::PlusOnePlusOne,
            }),
    )
}
