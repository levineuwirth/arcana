//! Sun-Crested Pterodon — `{4}{W}` 2/5 Dinosaur with Flying.
//! "This creature has vigilance as long as you control another Dinosaur."
//!
//! Flying is a base keyword. The conditional static keyword grant ("has
//! vigilance as long as you control another Dinosaur") has no primitive — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sun-Crested Pterodon");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: conditional static "has vigilance as long as you control another
    // Dinosaur" — no conditional static keyword grant primitive.
    reg.register(CardDefinition::new(name, chars))
}
