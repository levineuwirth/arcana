//! Dreg Mangler — `{1}{B}{G}` 3/3 Plant Zombie with Haste.
//! "Scavenge {3}{B}{G}" — the engine synthesizes the graveyard activated
//! ability from the keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreg Mangler");
    let plant = reg.interner_mut().intern("Plant");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Haste,
            KeywordAbility::Scavenge(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
