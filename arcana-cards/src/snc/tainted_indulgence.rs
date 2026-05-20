//! Tainted Indulgence — `{U}{B}` instant. "Draw two cards. Then
//! discard a card unless there are five or more mana values among
//! cards in your graveyard."
//!
//! GAP: 'number of distinct mana values among graveyard cards' is not
//! a script helper; conditional discard is dropped. Only the
//! unconditional draw is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tainted Indulgence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw two cards. Then discard a card unless there are five or more mana values among cards in your graveyard.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: 'five or more distinct mana values in your graveyard' has no script helper.
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
