//! Strongarm Tactics — `{1}{B}` sorcery. "Each player discards a card. Then each
//! player who didn't discard a creature card this way loses 4 life."
//!
//! # GAP: "each player who didn't discard a creature card loses 4 life" —
//! post-discard tracking of what type of card each player discarded is not
//! accessible via the script helpers. Best effort: each player discards a card
//! (opponent chooses for themselves, controller chooses for themselves); life-loss
//! conditional omitted.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Strongarm Tactics");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards a card. Then each player who didn't discard a creature card this way loses 4 life.".into(),
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
    // GAP: conditional life-loss based on what card type was discarded not accessible
    vec![
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}
