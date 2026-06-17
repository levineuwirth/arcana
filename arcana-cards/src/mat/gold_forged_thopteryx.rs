//! Gold-Forged Thopteryx — `{W}{U}` 1/3 Artifact Creature — Dinosaur Thopter.
//! Flying, lifelink.
//! Each legendary permanent you control has ward {2}.
//!
//! GAP: "Each legendary permanent you control has ward {2}" is a static
//! continuous ability granting a keyword to OTHER permanents you control. There
//! is no static-ability registration / keyword-grant-to-a-filtered-set primitive
//! in the available API for a creature card, so this clause is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gold-Forged Thopteryx");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(thopter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
