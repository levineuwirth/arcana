//! Phobian Phantasm — `{1}{B}{B}` 3/3 Illusion (B).
//!
//! * Flying; fear.
//! * Cumulative upkeep {B}.
//!
//! GAP (Cumulative upkeep): not a usable `KeywordAbility` variant and
//! the engine exposes no cumulative-upkeep mechanic (age counters +
//! escalating pay-or-sacrifice). Omitted; Flying and Fear are wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phobian Phantasm");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Fear],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
