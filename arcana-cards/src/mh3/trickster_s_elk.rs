//! Trickster's Elk — `{2}{G}` 3/3 green Enchantment Creature — Elk.
//!
//! Oracle:
//! * Bestow {1}{G} (GAP: Bestow is not an expressible keyword.)
//! * Enchanted creature loses all abilities and is a green Elk creature with
//!   base power and toughness 3/3. (GAP: bestow-aura attach static — only
//!   functions while bestowed/attached; not expressible as a triggered or
//!   activated ability.)
//!
//! Only bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trickster's Elk");
    let elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
