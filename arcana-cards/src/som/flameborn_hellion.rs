//! Flameborn Hellion — `{5}{R}` 5/4 Hellion with Haste.
//!
//! Oracle:
//! * Haste
//! * This creature attacks each combat if able. (static combat requirement — GAP)
//!
//! Haste is a base keyword. The "attacks each combat if able" static has no
//! expressible self-static must-attack primitive (Goad is a targeted,
//! end-of-turn effect, not a permanent self-requirement), so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flameborn Hellion");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static — "This creature attacks each combat if able."
    reg.register(CardDefinition::new(name, chars))
}
