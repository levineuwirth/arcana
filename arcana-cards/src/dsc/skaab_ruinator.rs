//! Skaab Ruinator — `{1}{U}{U}` 5/6 Zombie Horror with Flying.
//!
//! 1. "As an additional cost to cast this spell, exile three creature
//!    cards from your graveyard." — additional casting cost; GAP (no
//!    additional-cost field on the spell shape).
//! 2. Flying — base keyword, expressible.
//! 3. "You may cast this card from your graveyard." — cast-from-
//!    graveyard permission; GAP (no expressible primitive).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skaab Ruinator");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As an additional cost to cast this spell, exile three creature
    // cards from your graveyard" — no additional-cost field.
    // GAP: "You may cast this card from your graveyard" — no cast-from-
    // graveyard permission primitive.

    reg.register(CardDefinition::new(name, chars))
}
