//! Otarian Juggernaut — `{4}` 2/3 Artifact Creature — Juggernaut.
//!
//! * This creature can't be blocked by Walls. (Static blocking
//!   restriction — GAP'd.)
//! * Threshold — As long as there are seven or more cards in your
//!   graveyard, this creature gets +3/+0 and attacks each combat if
//!   able. (Static conditional buff + attack requirement — GAP'd.)
//!
//! `Threshold` is not in the expressible keyword surface → keywords
//! left empty.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Otarian Juggernaut");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "can't be blocked by Walls" — static, subtype-scoped block
    // restriction with no triggered/activated form.
    // GAP: Threshold static "+3/+0 and attacks each combat if able"
    // while 7+ cards in graveyard — pure conditional continuous effect.

    reg.register(CardDefinition::new(name, chars))
}
