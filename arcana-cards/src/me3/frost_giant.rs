//! Frost Giant — `{3}{R}{R}{R}` 4/4 Creature — Giant with Rampage 2.
//! Legends uncommon (1994); a mono-red Giant that grows significantly
//! larger when blocked by multiple creatures.
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
    let name = reg.interner_mut().intern("Frost Giant");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Rampage(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
