//! Keranos, God of Storms — `{3}{U}{R}` 6/5 Legendary Enchantment Creature —
//! God, with Indestructible (R/U).
//!
//! * Indestructible — keyword line.
//! * "As long as your devotion to blue and red is less than seven, Keranos
//!   isn't a creature." — GAP: a devotion-gated continuous type-changing
//!   static; not a triggered/activated ability.
//! * "Reveal the first card you draw on each of your turns. Whenever you reveal
//!   a land card this way, draw a card. Whenever you reveal a nonland card this
//!   way, Keranos deals 3 damage to any target." — GAP: there is no
//!   "reveal the first card drawn each turn" trigger, and the land/nonland
//!   branch keys off a reveal event that has no matching `TriggerCondition`.
//!   The whole reveal subsystem is unexpressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keranos, God of Storms");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: "As long as your devotion to blue and red is less than seven,
    // Keranos isn't a creature." — devotion-gated continuous type static.
    // GAP: "Reveal the first card you draw on each of your turns. Whenever you
    // reveal a land card this way, draw a card. Whenever you reveal a nonland
    // card this way, Keranos deals 3 damage to any target." — no reveal-first-
    // draw trigger / land-vs-nonland reveal branch in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
