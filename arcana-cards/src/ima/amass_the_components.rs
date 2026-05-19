//! Amass the Components — `{3}{U}` sorcery, "Draw three cards, then put a
//! card from your hand on the bottom of your library."
//!
//! # GAP
//! "Put a card from your hand on the bottom of your library" requires a
//! player-choice prompt for which hand card to move; there is no
//! Effect::PutFromHandToBottomOfLibrary or equivalent. DrawCards is fully
//! expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amass the Components");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards, then put a card from your hand on the bottom of your library.".into(),
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
    // GAP: player-choice put-hand-card-to-bottom-of-library not in Effect catalog
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
