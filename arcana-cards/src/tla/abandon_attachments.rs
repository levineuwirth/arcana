//! Abandon Attachments — `{1}{U/R}` instant — Lesson. "You may discard a card. If you do,
//! draw two cards."
//! The type line includes a subtype (Lesson) but the card is typed as Instant; subtypes on
//! instants are not stored in TypeLine. The conditional draw (if you discarded) is partially
//! expressible: discard 1 then draw 2 covers the "if you do" branch; the optional nature
//! (you may) is a GAP.
//! GAP: optional discard ("you may discard a card") — DiscardChoice has no optional/may variant;
//! Effect::Conditional condition for "did you just discard" not expressible.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abandon Attachments");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You may discard a card. If you do, draw two cards.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional discard ("you may discard a card") — DiscardChoice has no optional/may
    // variant; modeled as mandatory discard + draw
    vec![
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
