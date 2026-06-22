//! Dusk Charger — `{3}{B}` 3/3 black Horse.
//!
//! * Ascend — not in the supported `KeywordAbility` surface; GAP.
//! * "This creature gets +2/+2 as long as you have the city's blessing."
//!   — a pure conditional static continuous ability with no trigger or
//!   cost; not expressible on this shape. GAP.
//!
//! No triggered or activated abilities to emit; bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dusk Charger");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Ascend (city's blessing) keyword not in supported surface.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "+2/+2 as long as you have the city's blessing" — conditional
    // static continuous ability, not expressible on this shape.
    reg.register(CardDefinition::new(name, chars))
}
