//! Trapjaw Kelpie — `{4}{G/U}{G/U}` 3/3 Creature — Beast with Flash and
//! Persist.
//!
//! Oracle text:
//! * Flash — base keyword.
//! * Persist — base keyword (the dies → return-with-a-(-1/-1)-counter behavior
//!   is handled by the engine's Persist wiring).
//!
//! The hybrid `{G/U}` pips make the card both green and blue.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trapjaw Kelpie");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Persist],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
