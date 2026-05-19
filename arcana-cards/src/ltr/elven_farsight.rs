//! Elven Farsight — `{G}` sorcery. "Scry 3, then you may reveal the top card
//! of your library. If it's a creature card, draw a card."
//!
//! # GAP: conditional draw based on revealed top card's type ("if it's a
//!   creature card, draw") — no Effect variant for reveal-top-then-conditional-draw

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elven Farsight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry 3, then you may reveal the top card of your library. If it's a creature card, draw a card.".into(),
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
    // GAP: conditional draw based on revealed top card's type not in Effect catalog
    vec![Effect::Scry { player: entry.controller, count: 3 }]
}
