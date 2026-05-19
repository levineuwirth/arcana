//! Explosion of Riches — `{5}{R}` sorcery. "Draw a card. Each other player may draw a card.
//! Whenever a player draws a card this way, Explosion of Riches deals 5 damage to a random
//! opponent of that player."
//!
//! # GAP: On-draw trigger during resolution not in engine catalog.
//! # GAP: "each other player may draw" (optional per-player draw) not in engine catalog.
//! # GAP: Random opponent targeting not in engine catalog.
//! Partial: draw one card for the controller only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Explosion of Riches");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card. Each other player may draw a card. Whenever a player draws a card this way, Explosion of Riches deals 5 damage to a random opponent of that player.".into(),
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
    // GAP: on-draw trigger during resolution not in engine catalog
    // GAP: each-player-may-draw not in engine catalog
    // GAP: random opponent targeting not in engine catalog
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
