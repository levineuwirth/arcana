//! Fear of Being Hunted — `{1}{R}{R}` 4/2 Enchantment Creature — Nightmare.
//!
//! * Haste (keyword).
//! * "This creature must be blocked if able." — a static combat-requirement
//!   (lure-style). There is no must-be-blocked / lure Effect primitive, so the
//!   static is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of Being Hunted");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    // GAP: "This creature must be blocked if able" — no must-be-blocked / lure
    // primitive.
    reg.register(CardDefinition::new(name, chars))
}
