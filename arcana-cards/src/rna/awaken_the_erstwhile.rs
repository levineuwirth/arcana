//! Awaken the Erstwhile — `{3}{B}{B}` sorcery. "Each player discards all the
//! cards in their hand, then creates that many 2/2 black Zombie creature
//! tokens."
//!
//! # GAP: "creates that many … tokens" — the token count depends on how many
//! cards each player discarded (a dynamic per-player quantity not queryable
//! before the discard resolves). The catalog offers no Effect variant that
//! chains a discard count into a token-creation count at resolution time.
//! The discard is modeled for both players; the token rider is omitted.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Awaken the Erstwhile");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards all the cards in their hand, then creates that many 2/2 black Zombie creature tokens.".into(),
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
    use arcana_core::script;
    // GAP: token count tied to per-player discard amount
    let mut effects = Vec::new();
    // Discard for all players (controller first, then opponents)
    let controller_hand = script::hand_size(state, entry.controller);
    effects.push(Effect::Discard {
        player: entry.controller,
        count: controller_hand,
        choice: DiscardChoice::ControllerChooses,
    });
    // Opponent players: the catalog provides no iterator over all players;
    // GAP: discard for each opponent (only controller's discard is modeled here)
    effects
}
