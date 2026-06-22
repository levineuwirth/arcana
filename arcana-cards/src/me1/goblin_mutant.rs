//! Goblin Mutant — `{2}{R}{R}` 5/3 red Goblin Mutant.
//! Trample.
//! This creature can't attack if defending player controls an untapped creature
//! with power 3 or greater.
//! This creature can't block creatures with power 3 or greater.
//!
//! Trample is a base keyword. Both remaining lines are static combat
//! restrictions with no triggered/activated form — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Mutant");
    let goblin = reg.interner_mut().intern("Goblin");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "can't attack if defending player controls an untapped
    //      creature with power 3 or greater" — no expressible attack-restriction
    //      conditioned on the defender's board.
    // GAP: static "can't block creatures with power 3 or greater" — no
    //      expressible filtered block restriction.
    reg.register(CardDefinition::new(name, chars))
}
