//! Mage-Ring Bully — `{1}{R}` 2/2 Human Warrior.
//! "Prowess. This creature attacks each combat if able." Neither clause is
//! expressible with the demonstrated API, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mage-Ring Bully");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Prowess is not in the supported KeywordAbility surface.
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "This creature attacks each combat if able" is a static
    // attack-requirement on itself — no Effect/keyword expresses a
    // must-attack self-static.
    reg.register(CardDefinition::new(name, chars))
}
