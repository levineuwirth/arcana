//! Conspicuous Snoop — `{R}{R}` 2/2 Goblin Rogue.
//! "Play with the top card of your library revealed. You may cast Goblin
//! spells from the top of your library. As long as the top card of your
//! library is a Goblin card, this creature has all activated abilities of
//! that card."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conspicuous Snoop");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Play with the top card of your library revealed." — no
        // top-card-revealed static primitive.
        // GAP: "You may cast Goblin spells from the top of your library." —
        // no cast-from-top permission primitive.
        // GAP: "As long as the top card ... is a Goblin, this creature has
        // all activated abilities of that card." — no ability-granting-from-
        // top-card static primitive.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
