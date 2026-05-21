//! Peer Past the Veil — `{2}{R}{G}` instant. "Discard your hand. Then
//! draw X cards, where X is the number of card types among cards in your
//! graveyard."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::script;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peer Past the Veil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Discard your hand. Then draw X cards, where X is the number of card types among cards in your graveyard.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: X = "number of card types among cards in your graveyard" has no
    // script helper, so the X-card draw is omitted; the discard-your-hand
    // half is emitted.
    let hand = script::hand_size(state, entry.controller);
    if hand == 0 {
        return Vec::new();
    }
    vec![Effect::Discard {
        player: entry.controller,
        count: hand,
        choice: DiscardChoice::ControllerChooses,
    }]
}
