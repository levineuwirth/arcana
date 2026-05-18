//! Visionary Augmenter — `{2}{W}{W}` 2/1 Dwarf Artificer with Fabricate 2.
//! Fabricate is a keyword ability not represented in the demonstrated
//! KeywordAbility variants; keywords left empty for verify pipeline.
//!
//! # Rules references
//!
//! * CR 702.123 — Fabricate N. When this creature enters, put N +1/+1
//!   counters on it or create N 1/1 colorless Servo tokens. Not expressible
//!   with the demonstrated API.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Visionary Augmenter");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
