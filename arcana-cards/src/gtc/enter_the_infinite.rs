//! Enter the Infinite — `{8}{U}{U}{U}{U}` sorcery, "Draw cards equal to the
//! number of cards in your library, then put a card from your hand on top of
//! your library. You have no maximum hand size until your next turn."
//!
//! GAP: put a card from hand on top of library (player choice from hand,
//! then PutOnTopOfLibrary) not expressible via catalog without player choice.
//! GAP: no maximum hand size until your next turn not in catalog.
//! Emitting DrawCards equal to library size only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enter the Infinite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{U}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw cards equal to the number of cards in your library, then put a card from your hand on top of your library. You have no maximum hand size until your next turn.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: put a card from hand on top of library (player choice) not in catalog
    // GAP: no maximum hand size until next turn not in catalog
    let count = script::library_size(state, entry.controller);
    vec![Effect::DrawCards { player: entry.controller, count }]
}
