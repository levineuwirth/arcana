//! Undersea Invader — `{4}{U}{U}` 5/6 Creature — Giant Rogue. Mono-blue.
//!
//! Oracle:
//! - Flash — keyword.
//! - "This creature enters tapped." — GAP: enters-tapped is an entry
//!   replacement with no `Effect`/trigger API surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undersea Invader");
    let giant = reg.interner_mut().intern("Giant");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    // GAP: "This creature enters tapped." — entry replacement, no API surface.
    reg.register(CardDefinition::new(name, chars))
}
