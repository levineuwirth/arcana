//! Headstrong Brute — `{2}{R}` 3/3 Orc Pirate.
//! "This creature can't block." and "has menace as long as you control
//! another Pirate" are both pure static continuous abilities with no
//! trigger word and no cost; neither is expressible as a
//! triggered/activated ability, so both are GAP and only the bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Headstrong Brute");
    let orc = reg.interner_mut().intern("Orc");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "This creature can't block." — static continuous ability.
    // GAP: "This creature has menace as long as you control another
    // Pirate." — conditional static continuous ability.
    reg.register(CardDefinition::new(name, chars))
}
