//! Carnage Wurm — `{6}{G}` 6/6 Wurm.
//!
//! Oracle:
//! * "Bloodthirst 3" — if an opponent was dealt damage this turn, this
//!   creature enters with three +1/+1 counters on it. Parametrized
//!   keyword → `KeywordAbility::Bloodthirst(3)`.
//! * "Trample" → `KeywordAbility::Trample`.
//!
//! Both clauses are keyword abilities, so the entire card is the bones
//! plus a `keywords` vec — no triggered or activated abilities.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Carnage Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Bloodthirst(3), KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
