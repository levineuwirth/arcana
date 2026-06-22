//! Veteran Brawlers — `{1}{R}` 4/4 Human Soldier.
//! This creature can't attack if defending player controls an untapped land.
//! This creature can't block if you control an untapped land.
//!
//! Both lines are conditional static combat restrictions (no trigger word, no
//! cost). There is no expressible primitive for a board-state-conditioned
//! can't-attack / can't-block restriction, so both statics are GAP'd; the card
//! is emitted as its 4/4 body.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veteran Brawlers");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    // GAP: static "can't attack if defending player controls an untapped land"
    // — board-conditioned attack restriction is not expressible.
    // GAP: static "can't block if you control an untapped land" — board-
    // conditioned block restriction is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
