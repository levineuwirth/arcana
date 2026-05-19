//! Take Inventory — `{1}{U}` sorcery. "Draw a card, then draw cards equal
//! to the number of cards named Take Inventory in your graveyard."
//!
//! GAP: counting cards by name in graveyard is not expressible with
//! script helpers (no name-filter variant). Falling back to drawing 1
//! card (the guaranteed first draw); the conditional extra draws are
//! omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Take Inventory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card, then draw cards equal to the number of cards named Take Inventory in your graveyard.".into(),
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
    // GAP: count of cards named 'Take Inventory' in graveyard not expressible
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
