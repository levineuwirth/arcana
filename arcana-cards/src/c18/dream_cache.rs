//! Dream Cache — `{2}{U}` sorcery. "Draw three cards, then put two cards from
//! your hand both on top of your library or both on the bottom of your library."
//!
//! Draw is in catalog. The 'put two from hand on top/bottom' rider is not a
//! catalog primitive. Emit just the draw.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dream Cache");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards, then put two cards from your hand both on top of your library or both on the bottom of your library.".into(),
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
    // GAP: 'put two from hand on top or bottom' is a from-hand-to-library primitive
    // not in catalog.
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
