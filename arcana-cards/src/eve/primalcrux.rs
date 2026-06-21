//! Primalcrux — `{G}{G}{G}{G}{G}{G}` */* Elemental with Trample.
//! "Chroma — This creature's power and toughness are each equal to the
//!  number of green mana symbols in the mana costs of permanents you
//!  control."
//!
//! The Chroma characteristic-defining ability that sets the `*` value has
//! no demonstrated CDA registration hook for this shape, so P/T is left as
//! `*` (PtValue::Star) without the computing static.

// GAP (CDA): "power and toughness each equal to the number of green mana
// symbols in the mana costs of permanents you control" — no registration
// hook for a characteristic-defining power/toughness static.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Primalcrux");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{G}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
