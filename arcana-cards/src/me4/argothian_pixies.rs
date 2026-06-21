//! Argothian Pixies — `{1}{G}` 2/1 Creature — Faerie.
//!
//! Both printed abilities are pure statics with no triggered/activated
//! hook to install them from on this card class:
//! * "This creature can't be blocked by artifact creatures." — static
//!   conditional evasion; GAP'd.
//! * "Prevent all damage that would be dealt to this creature by
//!   artifact creatures." — static source-filtered prevention; GAP'd.
//! Emitted as faithful bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Argothian Pixies");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "can't be blocked by artifact creatures" and static
    // "prevent all damage from artifact creatures to this creature" —
    // no triggered/activated hook for printed statics on this class.

    reg.register(CardDefinition::new(name, chars))
}
