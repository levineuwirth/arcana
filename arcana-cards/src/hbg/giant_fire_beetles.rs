//! Giant Fire Beetles — `{2}{R}` 2/2 Insect with Menace, Double Team, and Conjure.
//! Alchemy card; has Menace (expressible) plus double team and conjure
//! (digital-only mechanics not representable in the current keyword API).
//! Best-effort: only Menace is encoded; verify pipeline should flag the gap.
//!
//! # Rules references
//!
//! * CR 702.110 — Menace. Can't be blocked except by two or more creatures.
//!   Engine wiring lives in the combat blocker-count filter.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Fire Beetles");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
