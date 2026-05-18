//! Arcbound Prototype — `{1}{W}` 0/0 Artifact Creature — Assembly-Worker
//! with Modular 2.
//! Modular is not in the demonstrated `KeywordAbility` API;
//! flagged for the verify pipeline. Keywords list is empty as a best-effort.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Prototype");
    let assembly_worker = reg.interner_mut().intern("Assembly-Worker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assembly_worker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
