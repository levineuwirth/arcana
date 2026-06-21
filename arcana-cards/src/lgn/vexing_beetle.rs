//! Vexing Beetle — `{4}{G}` 3/3 Insect.
//!
//! Oracle:
//!  * This spell can't be countered.
//!  * This creature gets +3/+3 as long as no opponent controls a creature.
//!
//! Both lines are static abilities, not triggered/activated abilities. Neither
//! is expressible via the MultiAbilityCreature surface (no "can't be countered"
//! marker, and the conditional continuous +3/+3 is a static layer effect) —
//! both are GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vexing Beetle");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    // GAP: "This spell can't be countered" — static cast-time restriction, no
    // expressible marker on this card class.
    // GAP: "gets +3/+3 as long as no opponent controls a creature" — conditional
    // continuous self-buff (static layer effect), not a triggered/activated
    // ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
