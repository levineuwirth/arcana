//! Duskhunter Bat — `{1}{B}` 1/1 Bat with Flying and Bloodthirst 1.
//!
//! Oracle:
//! * Bloodthirst 1 (If an opponent was dealt damage this turn, this creature
//!   enters with a +1/+1 counter on it.)
//! * Flying
//!
//! Both are base keywords; Bloodthirst is a parametrized keyword
//! (`KeywordAbility::Bloodthirst(1)`).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duskhunter Bat");
    let bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bloodthirst(1), KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
