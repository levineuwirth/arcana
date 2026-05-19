//! Journey for the Elixir — `{2}{G}` sorcery. "Search your library and
//! graveyard for a basic land card and a card named Jiang Yanggu, reveal them,
//! put them into your hand, then shuffle."
//!
//! # GAP: search graveyard simultaneously with library; named-card search
//! The engine's `TutorToHand` only searches the library; there is no
//! simultaneous graveyard search and no named-card filter.
//! Best-effort: tutor a basic land to hand only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Journey for the Elixir");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library and graveyard for a basic land card and a card named Jiang Yanggu, reveal them, put them into your hand, then shuffle.".into(),
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
    // GAP: no graveyard search and no named-card filter; partial: land to hand
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        reveal: true,
    }]
}
