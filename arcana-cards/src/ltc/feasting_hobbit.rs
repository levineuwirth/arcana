//! Feasting Hobbit — `{1}{G}` 2/2 Halfling Citizen.
//! Devour Food 3 (As this creature enters, you may sacrifice any number of
//! Foods. It enters with three times that many +1/+1 counters on it.)
//! Creatures with power less than this creature's power can't block it.
//!
//! GAP: the static "Creatures with power less than this creature's power
//! can't block it." is a power-comparison blocking restriction with no
//! demonstrated static / Effect primitive, so it is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feasting Hobbit");
    let halfling = reg.interner_mut().intern("Halfling");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Devour(3)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
