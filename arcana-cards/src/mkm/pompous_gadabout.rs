//! Pompous Gadabout — `{2}{G}` 4/2 Human Citizen.
//! "During your turn, this creature has hexproof." (conditional static)
//! "This creature can't be blocked by creatures that don't have a name."
//! Both lines are static continuous abilities that the MultiAbilityCreature
//! decomposition can't express, so the card is bones-only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pompous Gadabout");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "During your turn, this creature has hexproof" — conditional static.
    // GAP: "can't be blocked by creatures that don't have a name" — filtered
    //      can't-be-blocked static (no name-based block restriction primitive).
    reg.register(CardDefinition::new(name, chars))
}
