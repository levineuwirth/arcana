//! Brainsurge — `{2}{U}` instant. "Draw four cards, then put two cards from
//! your hand on top of your library in any order."
//!
//! GAP: "put two cards from your hand on top of your library in any order" —
//! no Effect variant for choosing cards from hand to put on library top.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brainsurge");
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
                text: "Draw four cards, then put two cards from your hand on top of your library in any order.".into(),
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
    // GAP: "put two cards from your hand on top of your library in any order" — no Effect variant for hand-to-library-top choice
    vec![
        Effect::DrawCards { player: entry.controller, count: 4 },
    ]
}
