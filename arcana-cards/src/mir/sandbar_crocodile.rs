//! Sandbar Crocodile — `{4}{U}` 6/5 Crocodile with Phasing.
//!
//! # Rules references
//!
//! * CR 702.25 — Phasing. This phases in or out before you untap
//!   during each of your untap steps. While it's phased out, it's
//!   treated as though it doesn't exist.
//!
//! NOTE: `Phasing` is not present in the demonstrated `KeywordAbility`
//! variant list. The `keywords` field is left empty so this file
//! compiles; the verify pipeline will flag the missing variant and a
//! human will route it.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandbar Crocodile");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
