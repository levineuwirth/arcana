//! Celestial Messenger — `{2}{U}{U}` 3/2 Bird Spirit with Flash and
//! Flying.
//! "This creature gets +1/+1 as long as you control a Yanling
//! planeswalker."
//!
//! The conditional static buff is a pure continuous ability (no
//! trigger, no cost) and is not expressible as a triggered/activated
//! ability — GAP'd. Only Flash, Flying, and the bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Celestial Messenger");
    let bird = reg.interner_mut().intern("Bird");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "This creature gets +1/+1 as long as you control a Yanling
    // planeswalker." — conditional static buff, not expressible as a
    // triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
