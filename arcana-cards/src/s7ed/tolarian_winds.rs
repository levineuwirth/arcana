//! Tolarian Winds — `{1}{U}` instant. "Discard all the cards in your
//! hand, then draw that many cards." Variable-count discard
//! (entire hand) and follow-on draw equal to count is dynamic via
//! script::hand_size before discard.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tolarian Winds");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard all the cards in your hand, then draw that many cards.".into(),
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
    let n = script::hand_size(state, entry.controller);
    vec![
        Effect::Discard { player: entry.controller, count: n, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: n },
    ]
}
