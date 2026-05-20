//! Heed the Mists — `{3}{U}{U}` sorcery. "Mill a card, then draw
//! cards equal to the milled card's mana value."
//!
//! The mill is expressed. The draw count is dynamic on the milled
//! card's mana value, which no script helper exposes — GAP the draw.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heed the Mists");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Mill a card, then draw cards equal to the milled card's mana value.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "draw cards equal to the milled card's mana value" — no
    // script helper exposes the just-milled card's mana value.
    vec![Effect::Mill { player: entry.controller, count: 1 }]
}
