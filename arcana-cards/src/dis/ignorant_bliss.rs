//! Ignorant Bliss — `{1}{R}` instant. Exile all cards from your hand
//! face down. At the beginning of the next end step, return those
//! cards to your hand, then draw a card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ignorant Bliss");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all cards from your hand face down. At the beginning of the next end step, return those cards to your hand, then draw a card.".into(),
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
    // GAP: "exile hand face down" + scheduled return-from-exile of cards in
    // hand zone is not expressible via the catalog (ExilePermanent / DelayedAction
    // operate on permanents/targets, not hand cards).
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
