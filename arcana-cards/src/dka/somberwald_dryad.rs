//! Somberwald Dryad — `{1}{G}` 2/2 Dryad with Forestwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Forestwalk). This creature can't be blocked
//!   as long as defending player controls a Forest.
//!
//! Forestwalk/Landwalk is not in the demonstrated KeywordAbility API;
//! this file is a best-effort stub. The verify pipeline will flag the gap.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Somberwald Dryad");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
