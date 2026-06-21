//! Crash of Rhino Beetles — `{4}{G}` 5/5 Insect with Trample.
//! "This creature gets +10/+10 as long as you control ten or more lands."
//!
//! GAP: the +10/+10 conditional static is a continuous (characteristic-
//! defining-style) ability with no trigger or cost — not expressible as a
//! triggered/activated ability on this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crash of Rhino Beetles");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
