//! Wolverine Pack — `{2}{G}{G}` 2/4 Creature — Wolverine with Rampage 2.
//! The Dark common (1994); a mono-green Wolverine that punishes opponents
//! who block with multiple creatures.
//!
//! # Rules references
//!
//! * CR 702.23 — Rampage N. Whenever this creature becomes blocked, it gets
//!   +N/+N until end of turn for each creature blocking it beyond the first.
//!   Engine wiring is handled by the `Rampage(N)` parametrized keyword variant.
//!
//! Rampage is a fully implemented parametrized keyword; listing it in `keywords`
//! as `KeywordAbility::Rampage(2)` is sufficient — the runtime pipeline does the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolverine Pack");
    let wolverine = reg.interner_mut().intern("Wolverine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolverine);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Rampage(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
