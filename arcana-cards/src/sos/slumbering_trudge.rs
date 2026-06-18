//! Slumbering Trudge — `{X}{G}` 6/6 Plant Beast. Enters with (3 − X)
//! stun counters; if X is 2 or less it enters tapped.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slumbering Trudge");
    let plant = reg.interner_mut().intern("Plant");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: "Enters with (3 − X) stun counters; if X is 2 or less, enters
    // tapped." This is an X-dependent enters-with replacement; there is no
    // primitive to seed counters/tapped state on entry as a function of the
    // spell's X. Omitted; emitting bones only.
    reg.register(CardDefinition::new(name, chars))
}
