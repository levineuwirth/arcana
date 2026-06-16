//! Undead Sprinter — `{B}{R}` 2/2 Zombie with Trample and Haste.
//!
//! * Trample, Haste (keywords).
//! * "You may cast this card from your graveyard if a non-Zombie creature died
//!   this turn. If you do, it enters with a +1/+1 counter." — the conditional
//!   graveyard-cast permission + counter rider is not expressible with the
//!   demonstrated API (no graveyard-cast permission Effect). GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undead Sprinter");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "You may cast this from your graveyard if a non-Zombie creature died
    // this turn; if you do it enters with a +1/+1 counter" — conditional cast-from-
    // graveyard permission is not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
