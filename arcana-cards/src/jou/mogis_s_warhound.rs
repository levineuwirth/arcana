//! Mogis's Warhound — `{1}{R}` 2/2 red Enchantment Creature — Dog.
//! Bestow {2}{R} is not in the available keyword surface and the
//! bestow alternate-cast / aura mechanic has no demonstrated primitive
//! (GAP). "This creature attacks each combat if able." is a static
//! attack requirement with no demonstrated primitive (GAP). "Enchanted
//! creature gets +2/+2 and attacks each combat if able." is the aura's
//! static buff/requirement, expressible only while bestowed and with no
//! demonstrated continuous-aura primitive here (GAP). Only the bones
//! remain.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mogis's Warhound");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Bestow {2}{R}" — not in the available keyword surface.
    // GAP: "This creature attacks each combat if able." — static attack
    // requirement, no demonstrated primitive.
    // GAP: "Enchanted creature gets +2/+2 and attacks each combat if
    // able." — continuous aura buff + requirement, no demonstrated form.
    reg.register(CardDefinition::new(name, chars))
}
