//! Doomsday — `{B}{B}{B}` sorcery. "Search your library and graveyard
//! for five cards and exile the rest. Put the chosen cards on top of
//! your library in any order. You lose half your life, rounded up."
//!
//! Dynamic half-life loss is computed. GAP: the library-restructure
//! primitive (search-five-and-exile-the-rest, place-on-top) isn't in
//! the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doomsday");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library and graveyard for five cards and exile the rest. Put the chosen cards on top of your library in any order. You lose half your life, rounded up.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let life = script::life(state, entry.controller).max(0) as u32;
    let half_up = (life + 1) / 2;
    // GAP: library/graveyard restructure (search-five-exile-rest, top-order).
    vec![Effect::LoseLife { player: entry.controller, amount: half_up }]
}
