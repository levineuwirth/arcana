//! Glint-Sleeve Artisan — `{2}{W}` 2/2 Dwarf Artificer with Fabricate 1.
//!
//! # Rules references
//!
//! * CR 702.123 — Fabricate. When this creature enters the battlefield,
//!   put a +1/+1 counter on it or create a 1/1 colorless Servo artifact
//!   creature token.
//!
//! NOTE: `Fabricate` is not present in the demonstrated `KeywordAbility`
//! variant list. The `keywords` field is left empty so this file
//! compiles; the verify pipeline will flag the missing variant and a
//! human will route it.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glint-Sleeve Artisan");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
