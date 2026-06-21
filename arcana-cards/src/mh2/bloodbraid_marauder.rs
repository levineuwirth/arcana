//! Bloodbraid Marauder — `{1}{R}` 3/1 Human Berserker.
//!
//! Oracle:
//! * This creature can't block. (static restriction)
//! * Delirium — This spell has cascade as long as there are four or more card
//!   types among cards in your graveyard.
//!
//! "Can't block" is a pure static self-restriction continuous ability — not
//! expressible as a triggered/activated ability (GAP). Delirium granting
//! cascade is a cast-time conditional keyword gate (not a triggered/activated
//! ability, and Delirium is not in the supported keyword surface) — GAP.
//! Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodbraid Marauder");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "This creature can't block." — pure static self-restriction.
    // GAP: "Delirium — this spell has cascade as long as 4+ card types in your
    //   graveyard" — cast-time conditional cascade grant; Delirium not in the
    //   supported keyword surface.
    reg.register(CardDefinition::new(name, chars))
}
