//! Change of Fortune — `{3}{R}` sorcery, "Discard your hand, then
//! draw a card for each card you've discarded this turn." Approximated
//! as: discard your whole hand, then draw that many.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Change of Fortune");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Discard your hand, then draw a card for each card you've discarded this turn.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Number to draw approximated by the hand size discarded now.
    let n = script::hand_size(state, entry.controller);
    vec![
        Effect::Discard {
            player: entry.controller,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: entry.controller, count: n },
    ]
}
