//! Unclaimed Cat — `{2}{W}` 3/3 Cat. Both abilities are
//! team-membership-conditional statics (Mirran → lifelink, Phyrexian →
//! Toxic 1); the team-affiliation game-state is not modeled, so both
//! conditional keyword grants are gaps. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unclaimed Cat");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: static "If you're on the Mirran team, ~ has lifelink" —
        // team affiliation is not modeled; conditional keyword grant omitted.
        // GAP: static "If you're on the Phyrexian team, ~ has Toxic 1" —
        // same; conditional keyword grant omitted.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
