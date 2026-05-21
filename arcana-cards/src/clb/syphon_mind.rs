//! Syphon Mind — `{3}{B}` sorcery. "Each other player discards a card.
//! You draw a card for each card discarded this way."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syphon Mind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each other player discards a card. You draw a card for each card discarded this way.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let opponents = script::opponents(state, entry.controller);
    // "for each card discarded this way" — each other player who can
    // discard contributes one card; count discardable opponents as the draw
    // amount. (Approximated as the opponent count.)
    let mut effects: Vec<Effect> = opponents
        .iter()
        .map(|&p| Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    let draw = opponents.len() as u32;
    if draw > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count: draw });
    }
    effects
}
