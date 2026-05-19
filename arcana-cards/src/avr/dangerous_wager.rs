//! Dangerous Wager — `{1}{R}` instant. "Discard your hand, then draw two cards."
//! GAP: "discard your hand" (discard all cards in hand) uses count that equals
//! hand size at resolution; approximated with a hand_size script call.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dangerous Wager");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard your hand, then draw two cards.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let hand = script::hand_size(state, entry.controller);
    vec![
        Effect::Discard {
            player: entry.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
