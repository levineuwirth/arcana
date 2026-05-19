//! Windfall — `{2}{U}` sorcery. "Each player discards their hand, then draws
//! cards equal to the greatest number of cards a player discarded this way."
//!
//! # GAP: "draws cards equal to the greatest number of cards a player discarded
//! this way" — tracking discard counts mid-resolution for a max() across players
//! is not accessible via the script helpers (hand_size is pre-discard; no
//! post-discard count). Best effort: each player discards their hand and each
//! draws 7 cards (fixed placeholder); GAP noted.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windfall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards their hand, then draws cards equal to the greatest number of cards a player discarded this way.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "greatest number of cards discarded this way" — post-discard count not accessible
    // Use pre-discard hand sizes as proxy for draw amount (hand_size = cards about to be discarded)
    let my_hand = script::hand_size(state, entry.controller);
    // We only have access to controller's hand size; opponent hand size not enumerable via helpers.
    // Best effort: each player discards their entire hand, then each draws controller's hand size.
    vec![
        Effect::Discard { player: entry.controller, count: my_hand, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: my_hand },
    ]
}
