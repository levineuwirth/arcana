//! Tragic Lesson — `{2}{U}` Instant. "Draw two cards. Then discard a
//! card unless you return a land you control to its owner's hand."
//!
//! # Implementation note
//! DrawCards 2 is expressible. The "unless return a land" conditional
//! discard (player chooses to bounce a land or discard instead) is not
//! expressible without a player-choice conditional.
//!
//! # GAP
//! "Discard unless return a land to hand" (player-choice conditional)
//! not in Effect catalog.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tragic Lesson");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. Then discard a card unless you return a land you control to its owner's hand.".into(),
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
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        // GAP: "discard unless return a land" player-choice conditional not expressible
    ]
}
