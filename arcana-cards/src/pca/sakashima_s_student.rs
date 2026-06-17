//! Sakashima's Student — `{2}{U}{U}` 0/0 Human Ninja.
//! "Ninjutsu {1}{U}. You may have this creature enter as a copy of any
//! creature on the battlefield, except it's a Ninja in addition to its other
//! creature types." Neither ability is expressible; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sakashima's Student");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Ninjutsu is not in the supported KeywordAbility surface.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enter as a copy of any creature on the battlefield" is an
    // as-enters copy replacement effect — not expressible with the
    // demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
