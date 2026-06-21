//! Nameless Race — `{3}{B}` */* Creature with Trample.
//!
//! Oracle:
//!  * Trample.
//!  * As this creature enters, pay any amount of life (capped at the number
//!    of white nontoken permanents your opponents control plus white cards in
//!    their graveyards).
//!  * Power and toughness are each equal to the life paid as it entered.
//!
//! Only the Trample keyword is expressible. The "pay any amount of life" ETB
//! cost is a player-chosen variable-life replacement whose cap is a board+
//! graveyard count, and the CDA "*/* equal to the life paid" both require
//! machinery the demonstrated API does not provide.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nameless Race");

    // GAP: "As this creature enters, pay any amount of life (capped by white
    // nontoken permanents opponents control + white cards in their graveyards)"
    // is a player-chosen variable-life ETB cost with a dynamic cap — not
    // expressible with the demonstrated Effect / ETB API.
    // GAP: "power and toughness are each equal to the life paid as it entered"
    // is a characteristic-defining ability tied to the (unmodeled) life paid;
    // bones carry */* as a fixed 0/0 placeholder.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
