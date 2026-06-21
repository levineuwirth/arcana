//! Seeker of Slaanesh — `{3}{R}` 3/3 Demon with Haste.
//!
//! Oracle:
//! * Haste.
//! * Allure of Slaanesh — Each opponent must attack with at least one
//!   creature each combat if able. (A static combat-restriction; not
//!   expressible with the demonstrated primitives — GAP'd.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seeker of Slaanesh");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Allure of Slaanesh — Each opponent must attack with at least
    // one creature each combat if able." is a static must-attack
    // combat restriction; no Effect/static primitive expresses it.

    reg.register(CardDefinition::new(name, chars))
}
