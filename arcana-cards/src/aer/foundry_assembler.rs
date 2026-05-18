//! Foundry Assembler — `{5}` 3/3 colorless Artifact Creature — Assembly-Worker.
//! Has Improvise (keyword not yet in engine API; best-effort stub).
//!
//! # Rules references
//!
//! * Improvise — Your artifacts can help cast this spell. Each artifact you tap
//!   after you're done activating mana abilities pays for {1}.
//!   Not expressible with current KeywordAbility variants;
//!   verify pipeline will flag for human routing.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Foundry Assembler");
    let assembly_worker = reg.interner_mut().intern("Assembly-Worker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assembly_worker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::default(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
