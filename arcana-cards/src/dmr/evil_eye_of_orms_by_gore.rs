//! Evil Eye of Orms-by-Gore — `{4}{B}` 3/6 Eye.
//! Both abilities are static and not expressible:
//!   GAP: "Non-Eye creatures you control can't attack" — global attack
//!        restriction static.
//!   GAP: "This creature can't be blocked except by Walls" — conditional
//!        unblockable static (Effect::CantBeBlocked is unconditional only).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evil Eye of Orms-by-Gore");
    let eye = reg.interner_mut().intern("Eye");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eye);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
