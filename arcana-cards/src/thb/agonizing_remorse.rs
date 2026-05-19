//! Agonizing Remorse — `{1}{B}` sorcery, "Target opponent reveals their hand.
//! You choose a nonland card from it or a card from their graveyard. Exile that
//! card. You lose 1 life."
//!
//! GAP: revealing opponent's hand and choosing a specific card to exile from
//! hand or graveyard (targeted hand/graveyard inspection + conditional exile)
//! is not expressible. Partial: lose 1 life only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Agonizing Remorse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You choose a nonland card from it or a card from their graveyard. Exile that card. You lose 1 life.".into(),
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
    // GAP: reveal opponent's hand and choose a card to exile from hand or graveyard
    vec![Effect::LoseLife { player: entry.controller, amount: 1 }]
}
