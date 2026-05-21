//! Doomsday — `{B}{B}{B}` sorcery. "Search your library and graveyard
//! for five cards and exile the rest. Put the chosen cards on top of
//! your library in any order. You lose half your life, rounded up."
//!
//! The library/graveyard rebuild has no engine primitive. Only the
//! life loss is expressed; the search-and-exile-rest is a GAP.

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
    // GAP: search-five / exile-the-rest / rebuild-library has no
    // engine primitive; only the life loss is emitted.
    let life = script::life(state, entry.controller).max(0) as u32;
    let half_up = (life + 1) / 2;
    vec![Effect::LoseLife { player: entry.controller, amount: half_up }]
}
