//! Predatory Wurm — `{3}{G}` 4/4 Creature — Wurm.
//!
//! Oracle:
//! * Vigilance
//! * This creature gets +2/+2 as long as you control a Garruk
//!   planeswalker. — GAP (conditional static continuous boost)
//!
//! Vigilance is a base keyword. The conditional +2/+2 is a pure static
//! continuous effect with no triggered/activated form.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predatory Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "This creature gets +2/+2 as long as you control a Garruk
    //      planeswalker." — conditional static continuous P/T boost.
    reg.register(CardDefinition::new(name, chars))
}
