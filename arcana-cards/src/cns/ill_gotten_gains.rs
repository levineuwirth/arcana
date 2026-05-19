//! Ill-Gotten Gains — `{2}{B}{B}` sorcery.
//! "Exile Ill-Gotten Gains. Each player discards their hand, then returns up to three cards from their graveyard to their hand."
//! GAP: "each player" iteration (not just controller); "exile this spell" self-exile; "return up to three cards from graveyard to hand" with player choice are inexpressible.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ill-Gotten Gains");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile Ill-Gotten Gains. Each player discards their hand, then returns up to three cards from their graveyard to their hand.".into(),
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
    // GAP: "exile this spell", "each player" loop, and graveyard-to-hand choice are inexpressible;
    // partial: discard controller's hand only
    let hand = script::hand_size(state, entry.controller);
    vec![Effect::Discard {
        player: entry.controller,
        count: hand,
        choice: DiscardChoice::ControllerChooses,
    }]
}
