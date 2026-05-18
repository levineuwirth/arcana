//! Glowering Rogon — `{5}{G}` 4/4 Creature — Beast with Amplify 1.
//!
//! Legions (2003). As it enters, its controller may reveal any number of
//! Beast cards from their hand; it enters with a +1/+1 counter for each
//! revealed card (Amplify 1).
//!
//! # Rules references
//!
//! * CR 702.37 — Amplify. As this creature enters the battlefield, reveal
//!   any number of cards of the creature's subtype(s) from your hand and put
//!   N +1/+1 counters on it for each card revealed (N=1 here). Engine wiring
//!   handles the ETB reveal-and-counter mechanic.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glowering Rogon");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Amplify(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
