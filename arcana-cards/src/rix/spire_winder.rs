//! Spire Winder — `{3}{U}` 2/3 Creature — Snake.
//!
//! Oracle:
//! * Flying.
//! * Ascend — GAP: Ascend is not on the supported keyword surface (no
//!   city's-blessing mechanic).
//! * This creature gets +1/+1 as long as you have the city's blessing. —
//!   GAP: static conditioned-on-city's-blessing pump is not expressible
//!   (no city's-blessing state condition / static self-pump primitive).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spire Winder");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
