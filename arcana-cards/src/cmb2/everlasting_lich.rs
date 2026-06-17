//! Everlasting Lich — `{2}{B}{B}{B}` 4/0 Zombie.
//! "Everlasting Lich can't block."
//! "Everlasting Lich can't die." (indestructible, can't be sacrificed, and
//!  doesn't die to having 0 toughness).
//!
//! Both lines are pure continuous statics with no trigger word or activation
//! cost. "Can't block" has no static keyword/Effect form here, and "can't die"
//! is a bespoke replacement static — both are GAP'd. Only bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Everlasting Lich");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: static — "can't block" (no static can't-block form here).
    // GAP: static — "can't die" (bespoke replacement: indestructible +
    // can't-be-sacrificed + ignores 0-toughness SBA; no keyword/Effect form).
    reg.register(CardDefinition::new(name, chars))
}
