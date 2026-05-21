//! Pulse of the Grid — `{1}{U}{U}` instant. "Draw two cards, then
//! discard a card. Then if an opponent has more cards in hand than
//! you, return Pulse of the Grid to its owner's hand."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pulse of the Grid");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards, then discard a card. Then if an opponent has more cards in hand than you, return Pulse of the Grid to its owner's hand.".into(),
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
    // GAP: 'Then if an opponent has more cards in hand than you, return Pulse
    // of the Grid to its owner's hand' — an instant returning ITSELF (a stack
    // object) to hand on resolution is not expressible; ReturnToHand operates
    // on permanents, not on the resolving spell. The hand-comparison condition
    // also lacks an Effect::Conditional condition variant in the catalog.
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
