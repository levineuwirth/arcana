//! Faceless One — `{5}` 3/3 Legendary Enchantment Creature — Background.
//!
//! Oracle:
//! * "If Faceless One is your commander, choose a color before the game
//!   begins. Faceless One is the chosen color." — a commander-format
//!   color-identity setup ability; the engine models neither the
//!   commander zone nor a "choose a color before the game" hook, so this
//!   is GAP'd (no trigger/activated/static primitive applies).
//! * "Choose a Background" — the Background partner-commander mechanic
//!   (a deck-building permission, not an on-battlefield ability) is not
//!   modeled by the engine; GAP'd. The Scryfall-parsed keyword
//!   "Choose a background" has no `KeywordAbility` variant.
//!
//! Bones (mana cost, colors, type line, P/T) are faithful; the card's
//! only printed text is two commander-format statics that have no
//! expressible primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Faceless One");
    let background = reg.interner_mut().intern("Background");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(background);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword — "Choose a background" is the Background partner-
        // commander deckbuilding mechanic, no KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "If Faceless One is your commander, choose a color
    // before the game begins; Faceless One is the chosen color." No
    // commander-zone / pregame-color-choice hook is modeled.
    // GAP: static — "Choose a Background" (partner-commander permission).
    reg.register(CardDefinition::new(name, chars))
}
