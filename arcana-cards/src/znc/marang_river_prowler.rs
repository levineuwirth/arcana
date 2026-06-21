//! Marang River Prowler — `{2}{U}` 2/1 Creature — Human Rogue.
//!
//! Oracle:
//! * This creature can't block and can't be blocked. (Static combat
//!   restriction — no trigger/cost; GAP'd.)
//! * You may cast this card from your graveyard as long as you control a black
//!   or green permanent. (Static casting-permission — GAP'd.)
//!
//! Both lines are pure statics / casting permissions with no triggered or
//! activated ability to decompose, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marang River Prowler");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "This creature can't block and can't be blocked." — a self-applied
    // static combat restriction; no continuous-static hook on this card class.
    // GAP: "You may cast this card from your graveyard as long as you control a
    // black or green permanent." — a conditional casting permission; no
    // alternate-cast-zone mechanic in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
