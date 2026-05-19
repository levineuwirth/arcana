//! Vicious Rumors — `{B}` sorcery, "Vicious Rumors deals 1 damage to each
//! opponent. Each opponent discards a card, then mills a card. You gain 1 life."
//!
//! GAP: "each opponent" iteration (deal damage to / discard / mill for each
//! opponent individually) requires iterating over opponent player ids, which
//! is not expressible with a single catalog Effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vicious Rumors");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Vicious Rumors deals 1 damage to each opponent. Each opponent discards a card, then mills a card. You gain 1 life.".into(),
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
    // GAP: iterating "each opponent" to deal damage, discard, and mill per opponent
    // (no catalog Effect applies damage/discard/mill to each opponent simultaneously)
    vec![Effect::GainLife { player: entry.controller, amount: 1 }]
}
