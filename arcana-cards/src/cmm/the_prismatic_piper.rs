//! The Prismatic Piper — `{5}` 3/3 Legendary Shapeshifter (and Partner).
//!
//! Oracle:
//! * If The Prismatic Piper is your commander, choose a color before the game
//!   begins; it is the chosen color. (Commander-format color-choice static —
//!   not expressible; GAP.)
//! * Partner. (Partner is not an expressible keyword — omitted; GAP.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Prismatic Piper");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "choose a color before the game; it is the chosen color" (commander
    //      color-identity static) and Partner are both not expressible.
    reg.register(CardDefinition::new(name, chars))
}
