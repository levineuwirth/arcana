//! Vizier of the Menagerie — `{3}{G}` 3/4 Snake Cleric.
//!
//! Oracle (all three are continuous/permission statics with no engine
//! primitive — GAP'd; no keyword line, no triggers, no activations):
//! * You may look at the top card of your library any time.
//! * You may cast creature spells from the top of your library.
//! * You can spend mana of any type to cast creature spells.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vizier of the Menagerie");
    let snake = reg.interner_mut().intern("Snake");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "You may look at the top card of your library any time."
    // GAP: "You may cast creature spells from the top of your library."
    // GAP: "You can spend mana of any type to cast creature spells."
    //   All three are permission/cost-altering statics with no primitive.

    reg.register(CardDefinition::new(name, chars))
}
