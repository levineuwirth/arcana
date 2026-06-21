//! Marble Priest — `{5}` 3/3 Artifact Creature — Cleric.
//! "All Walls able to block this creature do so."
//! "Prevent all combat damage that would be dealt to this creature by
//! Walls."
//!
//! Both lines are pure static abilities. Neither is expressible with the
//! available triggered/activated primitives, so only the bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marble Priest");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cleric);

    // GAP: static "All Walls able to block this creature do so." — a
    // lure/force-block static over a subtype-filtered set is not expressible.
    // GAP: static "Prevent all combat damage that would be dealt to this
    // creature by Walls." — a permanent (no-duration) source-filtered
    // prevention static is not expressible with the available primitives.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
