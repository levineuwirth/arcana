//! Union of the Third Path — `{2}{W}` instant.
//! "Draw a card, then you gain life equal to the number of cards in your hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Union of the Third Path");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card, then you gain life equal to the number of cards in your hand.".into(),
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
    // Draw happens first; hand_size is read before drawing (+1 to account for the drawn card).
    // Per oracle text order: draw first, then gain life equal to cards in hand.
    // We read hand size AFTER the draw would have happened — but script runs at resolution
    // before any effect resolves. Per catalog convention, we read current hand_size and
    // add 1 for the card being drawn.
    let hand = script::hand_size(state, entry.controller) + 1;
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::GainLife { player: entry.controller, amount: hand },
    ]
}
