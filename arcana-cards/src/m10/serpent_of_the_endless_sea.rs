//! Serpent of the Endless Sea — `{4}{U}` */* Serpent.
//! "Serpent of the Endless Sea's power and toughness are each equal to the
//!  number of Islands you control.
//!  This creature can't attack unless defending player controls an Island."
//!
//! GAP: the characteristic-defining */* (power/toughness = Islands you
//! control) is recorded as PtValue::Star, but no effect/ability surface wires
//! the CDA computation here; and "can't attack unless defending player
//! controls an Island" is a static attack restriction with no
//! triggered/activated expression. Both are noted gaps.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serpent of the Endless Sea");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
