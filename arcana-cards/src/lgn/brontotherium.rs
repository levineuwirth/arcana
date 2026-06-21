//! Brontotherium — `{4}{G}{G}` 5/3 Beast with Trample and Provoke.
//! "Trample
//!  Provoke (Whenever this creature attacks, you may have target creature
//!  defending player controls untap and block it if able.)"
//!
//! Both abilities are evergreen/parametrized keywords — Trample and
//! Provoke are KeywordAbility variants, so nothing beyond listing them is
//! required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brontotherium");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Provoke],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
