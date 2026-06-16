//! Scourge Wolf — `{R}{R}` 2/2 Wolf Horror with First strike.
//! Delirium — has double strike as long as there are four or more
//! card types among cards in your graveyard.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge Wolf");
    let wolf = reg.interner_mut().intern("Wolf");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: Delirium static — "has double strike as long as four or more
    // card types among cards in your graveyard" is a conditional
    // continuous self-keyword grant, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
