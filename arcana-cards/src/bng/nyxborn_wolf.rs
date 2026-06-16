//! Nyxborn Wolf — `{2}{G}` 3/1 green Enchantment Creature — Wolf.
//! Bestow {4}{G} (GAP — Bestow not in usable keyword surface).
//! Enchanted creature gets +3/+1. (GAP — aura static; only relevant when cast
//! for bestow cost, which is not expressible here.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxborn Wolf");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: Bestow {4}{G} — Bestow is not in the usable keyword surface.
    // GAP: "Enchanted creature gets +3/+1." — aura attach static; not
    // expressible on this card class.
    reg.register(CardDefinition::new(name, chars))
}
