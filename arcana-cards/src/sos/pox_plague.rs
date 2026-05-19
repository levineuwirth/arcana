//! Pox Plague — `{B}{B}{B}{B}{B}` sorcery. "Each player loses half their
//! life, then discards half the cards in their hand, then sacrifices half
//! the permanents they control of their choice. Round down each time."
//!
//! GAP: 'each player' iteration (not just controller or a single target)
//! not expressible. GAP: 'sacrifice half the permanents, controller's
//! choice' not in catalog. Approximated with script helpers for controller
//! only: lose half life, discard half hand.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pox Plague");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player loses half their life, then discards half the cards in their hand, then sacrifices half the permanents they control of their choice. Round down each time.".into(),
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
    // GAP: 'each player' iteration not expressible; controller only
    // GAP: 'sacrifice half permanents of choice' not in catalog
    let life = script::life(state, entry.controller);
    let lose_life = (life / 2).max(0) as u32;
    let hand = script::hand_size(state, entry.controller);
    let discard = hand / 2;
    let mut effects = Vec::new();
    if lose_life > 0 {
        effects.push(Effect::LoseLife { player: entry.controller, amount: lose_life });
    }
    if discard > 0 {
        effects.push(Effect::Discard { player: entry.controller, count: discard, choice: DiscardChoice::ControllerChooses });
    }
    effects
}
