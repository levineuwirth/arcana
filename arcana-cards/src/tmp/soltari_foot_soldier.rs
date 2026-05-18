//! Soltari Foot Soldier — `{W}` 1/1 Soltari Soldier with Shadow.
//! A Tempest-block common; Shadow restricts blocking to only other
//! Shadow creatures, making this an evasive one-drop in white.
//!
//! # Rules references
//!
//! * CR 702.27 — Shadow. A creature with shadow can block or be
//!   blocked only by creatures that also have shadow.
//!
//! NOTE: `KeywordAbility::Shadow` is not present in the currently
//! demonstrated API surface. The keyword list is left empty as a
//! best-effort stub; the verify pipeline will flag this gap for a
//! human to wire the Shadow variant when it is added to the engine.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soltari Foot Soldier");
    let soltari = reg.interner_mut().intern("Soltari");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soltari);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
