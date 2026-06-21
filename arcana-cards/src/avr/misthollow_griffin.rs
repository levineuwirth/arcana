//! Misthollow Griffin — `{2}{U}{U}` 3/3 Creature — Griffin.
//! Flying.
//! "You may cast this card from exile."
//!
//! Flying is a base keyword. The cast-from-exile permission is a static
//! casting-permission ability with no demonstrated `Effect` / activated /
//! triggered representation on this card class — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Misthollow Griffin");
    let griffin = reg.interner_mut().intern("Griffin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "You may cast this card from exile." No casting-zone
    // permission primitive available for this card class.
    reg.register(CardDefinition::new(name, chars))
}
