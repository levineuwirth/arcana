//! Interpret the Signs — `{5}{U}` sorcery. Scry 3, then reveal the top
//! card of your library. Draw cards equal to that card's mana value.
//! (Cannot inspect a single top-library card's CMC — emit only the
//! scry.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Interpret the Signs");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Scry 3, then reveal the top card of your library. Draw cards equal to that card's mana value.".into(),
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
    // GAP: "reveal the top card of your library, draw cards equal to that card's mana value"
    // — no helper to inspect the top library card's mana value.
    vec![Effect::Scry {
        player: entry.controller,
        count: 3,
    }]
}
