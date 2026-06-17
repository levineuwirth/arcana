//! Sustaining Spirit — `{1}{W}` 0/3 Angel Spirit.
//! "Cumulative upkeep {1}{W}." (not an expressible keyword — GAP)
//! "Damage that would reduce your life total to less than 1 reduces it to 1
//! instead." (static replacement — GAP)
//!
//! Neither ability is expressible with the available primitives; only the
//! bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sustaining Spirit");
    let angel = reg.interner_mut().intern("Angel");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: Cumulative upkeep {1}{W} — not an expressible keyword.
    // GAP: static replacement "damage reducing your life below 1 reduces it to 1 instead" — no such replacement primitive.
    reg.register(CardDefinition::new(name, chars))
}
