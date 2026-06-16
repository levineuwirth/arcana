//! Watchdog — `{3}` 1/2 Artifact Creature — Dog.
//! "This creature blocks each combat if able.
//!  As long as this creature is untapped, all creatures attacking you get
//!  -1/-0."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Watchdog");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    // GAP: static "blocks each combat if able" — no forced-block self-static.
    // GAP: static "as long as ~ is untapped, all creatures attacking you get
    // -1/-0" — a tap-conditional board-wide debuff is not expressible here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
