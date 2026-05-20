//! Ill-Gotten Gains — `{2}{B}{B}` sorcery. "Exile Ill-Gotten Gains. Each
//! player discards their hand, then returns up to three cards from their
//! graveyard to their hand." Exile-self has no Effect variant; the per-player
//! graveyard-return is not expressible. We emit the per-player discard via
//! Sequence over `script::all_players`, dynamically sized to each player's
//! hand.

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile Ill-Gotten Gains. Each player discards their hand, then returns up to three cards from their graveyard to their hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no Effect for "exile this spell from the stack on resolution" and no
    // per-player graveyard tutor; only the each-player discards is emitted.
    let seq = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Discard {
            player: p,
            count: script::hand_size(state, p),
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    vec![Effect::Sequence(seq)]
}
