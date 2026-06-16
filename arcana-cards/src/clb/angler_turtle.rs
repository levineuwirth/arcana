//! Angler Turtle — `{5}{U}{U}` 5/7 blue Turtle with Hexproof.
//!
//! Oracle:
//! * Hexproof.
//! * Creatures your opponents control attack each combat if able.
//!
//! The Hexproof keyword is a base characteristic. The "attack each
//! combat if able" static is a continuous combat-restriction that the
//! demonstrated API cannot express (no Goad-all / forced-attack static),
//! so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angler Turtle");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    // GAP: static "Creatures your opponents control attack each combat if
    // able" — a board-wide forced-attack restriction with no demonstrated
    // effect/static primitive.
    reg.register(CardDefinition::new(name, chars))
}
