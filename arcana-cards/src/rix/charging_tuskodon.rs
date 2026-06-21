//! Charging Tuskodon — `{3}{R}{R}` 4/4 Dinosaur. Trample.
//! "If this creature would deal combat damage to a player, it deals
//! double that damage to that player instead" is a self-damage
//! replacement static with no primitive in the demonstrated API — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charging Tuskodon");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: replacement static "deals double combat damage to a player" — no
    // combat-damage doubling / replacement primitive in the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
