//! Wheel of Fortune — `{2}{R}` sorcery. "Each player discards their hand,
//! then draws seven cards."
//!
//! Hand-sized discard is not a literal — use `script::hand_size` for each
//! player's count, then push a draw-seven.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wheel of Fortune");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards their hand, then draws seven cards.".into(),
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
    let mut effects: Vec<Effect> = Vec::new();
    for p in script::all_players(state) {
        let n = script::hand_size(state, p);
        if n > 0 {
            effects.push(Effect::Discard {
                player: p,
                count: n,
                choice: DiscardChoice::ControllerChooses,
            });
        }
    }
    for p in script::all_players(state) {
        effects.push(Effect::DrawCards { player: p, count: 7 });
    }
    let _ = entry;
    effects
}
