//! Heartwarming Redemption — `{2}{R}{W}` instant, "Discard all the cards in
//! your hand, then draw that many cards plus one. You gain life equal to the
//! number of cards in your hand."
//!
//! # GAP
//! "Discard all" (hand_size-count discard) followed by draw (hand_size+1)
//! then gain life equal to new hand size are sequentially dependent on mutable
//! hand counts. Discard all and draw that-many+1 are expressed via helpers;
//! gain life equal to new hand size requires a separate hand_size call which
//! reflects the post-draw count, which is not available at resolve time here.
//! Using hand_size before discarding for draw count; life gain uses a fixed
//! conservative best-effort (1 for the +1 draw).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heartwarming Redemption");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard all the cards in your hand, then draw that many cards plus one. You gain life equal to the number of cards in your hand.".into(),
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
    let hand = script::hand_size(state, entry.controller);
    let draw = hand + 1;
    vec![
        Effect::Discard { player: entry.controller, count: hand, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: draw },
        // GAP: life gain equal to new hand size after draw is not computable
        // here (post-draw hand size is unknown at resolution time); using draw
        // count as proxy.
        Effect::GainLife { player: entry.controller, amount: draw },
    ]
}
