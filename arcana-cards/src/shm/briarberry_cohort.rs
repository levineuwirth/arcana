//! Briarberry Cohort — `{1}{U}` 1/1 blue Faerie Soldier.
//! Flying.
//! This creature gets +1/+1 as long as you control another blue creature.
//!
//! The keyword (Flying) is a base characteristic. The conditional static
//! "+1/+1 as long as you control another blue creature" is a continuous
//! self-buff — not a triggered or activated ability — and cannot be
//! expressed via the MultiAbilityCreature surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Briarberry Cohort");
    let faerie = reg.interner_mut().intern("Faerie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: static "gets +1/+1 as long as you control another blue creature"
    // is a conditional continuous self-buff, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
