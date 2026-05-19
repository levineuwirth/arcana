//! Frantic Inventory — `{1}{U}` instant. "Draw a card, then draw cards equal
//! to the number of cards named Frantic Inventory in your graveyard."
//!
//! # GAP: draw count scaled to the number of copies of a specific named card
//! in your graveyard requires dynamic state lookup not supported by
//! DrawCards (fixed count). Best-effort: draw 1 card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frantic Inventory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card, then draw cards equal to the number of cards named Frantic \
                       Inventory in your graveyard."
                    .into(),
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
    // GAP: additional draws scaled to named-card graveyard count not supported
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
