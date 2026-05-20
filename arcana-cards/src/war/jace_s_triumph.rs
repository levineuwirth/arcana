//! Jace's Triumph — `{2}{U}` sorcery. "Draw two cards. If you
//! control a Jace planeswalker, draw three cards instead."
//!
//! GAP: no way to test control of a planeswalker by sub-name "Jace";
//! only the unconditional two-card draw is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace's Triumph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw two cards. If you control a Jace planeswalker, draw three cards instead.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if you control a Jace planeswalker" conditional draw — no
    // planeswalker-subtype control test in scripting catalog.
    vec![Effect::DrawCards {
        player: entry.controller,
        count: 2,
    }]
}
