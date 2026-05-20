//! Congregation at Dawn — `{G}{G}{W}` instant. "Search your library
//! for up to three creature cards, reveal them, then shuffle and put
//! those cards on top in any order."
//!
//! No "search and put on top of library" primitive (TutorToHand /
//! TutorToBattlefield are the only search destinations). Emitting the
//! closest expressible form: search up to three creatures to hand.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Congregation at Dawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to three creature cards, \
                   reveal them, then shuffle and put those cards on top \
                   in any order."
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
    // GAP: no "search and put on top of library" primitive; emitting a
    // single creature tutor-to-hand as the closest approximation (the
    // up-to-three count and on-top placement are not modeled).
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        reveal: true,
    }]
}
