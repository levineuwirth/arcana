//! Largepox — `{B}{B}{B}{B}` sorcery. Each player discards, loses 1 life,
//! sacrifices one of each permanent type, exiles a graveyard card, mills 1,
//! removes a counter, and gets a poison counter.
//!
//! Best effort: discard and lose-life for each player (controller only, as
//! we lack iteration over all players). Sacrifice, exile-from-graveyard,
//! mill, remove-counter, and poison-counter effects are all GAPs.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Largepox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards a card, then loses 1 life, then sacrifices an artifact, a creature, an enchantment, a land, a planeswalker, and a tribal permanent, then exiles a card from their graveyard, then puts the top card of their library into their graveyard, then removes a counter from a permanent they control, then gets a poison counter.".into(),
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
    // Best effort: controller only; all-player iteration is a GAP.
    // GAP: sacrifice one of each permanent type, exile from graveyard,
    //      mill 1, remove counter, poison counter for each player
    vec![
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::LoseLife { player: entry.controller, amount: 1 },
        Effect::Mill { player: entry.controller, count: 1 },
    ]
}
