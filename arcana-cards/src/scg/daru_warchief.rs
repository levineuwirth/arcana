//! Daru Warchief — `{2}{W}{W}` 1/1 Human Soldier.
//! "Soldier spells you cast cost {1} less to cast." and "Soldier
//! creatures you control get +1/+2."
//!
//! Both abilities are static continuous effects (a cost reduction and a
//! filtered anthem). Neither is a triggered or activated ability and
//! neither is expressible with the demonstrated primitives, so both are
//! GAP'd. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "Soldier spells you cast cost {1} less to cast." — static cost
//      reduction; not modeled.
// GAP: "Soldier creatures you control get +1/+2." — static filtered anthem;
//      not a triggered/activated ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daru Warchief");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
