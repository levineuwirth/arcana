//! Party Crasher — `{4}{R}` 3/3 Goblin Berserker with Haste.
//!
//! * Haste — keyword line.
//! * "You can attack with this creature once each combat during each
//!   opponent's turn." — GAP: a continuous static combat-permission (attacking
//!   on opponents' turns); not a triggered/activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Party Crasher");
    let goblin = reg.interner_mut().intern("Goblin");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "You can attack with this creature once each combat during each
    // opponent's turn." — continuous static combat-permission; not expressible.

    reg.register(CardDefinition::new(name, chars))
}
