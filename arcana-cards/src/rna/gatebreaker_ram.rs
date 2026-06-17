//! Gatebreaker Ram — `{2}{G}` 2/2 Sheep.
//! "This creature gets +1/+1 for each Gate you control." (dynamic static — GAP)
//! "As long as you control two or more Gates, it has vigilance and trample."
//! (conditional static — GAP)
//!
//! Both lines are static continuous abilities with no trigger word and no
//! activation cost, so neither maps to a TriggeredAbilityDef / ActivatedAbilityDef.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gatebreaker Ram");
    let sheep = reg.interner_mut().intern("Sheep");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sheep);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "+1/+1 for each Gate you control" — dynamic static continuous P/T.
    // GAP: "while you control two or more Gates, has vigilance and trample" —
    //      conditional static keyword grant.
    reg.register(CardDefinition::new(name, chars))
}
