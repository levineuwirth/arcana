//! Headless Skaab — `{2}{U}` 3/6 Zombie Warrior.
//! As an additional cost to cast this spell, exile a creature card from your
//! graveyard. (GAP: additional cast cost not expressible.)
//! This creature enters tapped. (GAP: enters-tapped replacement not
//! expressible.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Headless Skaab");
    let zombie = reg.interner_mut().intern("Zombie");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: additional cast cost (exile a creature card from graveyard) not
    // expressible. GAP: enters tapped — replacement effect not expressible.

    reg.register(CardDefinition::new(name, chars))
}
