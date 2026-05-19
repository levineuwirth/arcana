//! Search for Glory — `{2}{W}` snow sorcery. "Search your library for a
//! snow permanent card, a legendary card, or a Saga card, reveal it, put
//! it into your hand, then shuffle. You gain 1 life for each {S} spent to
//! cast this spell."
//!
//! GAP: searching for snow/legendary/Saga cards by supertype not
//! expressible with ObjectFilter. GAP: life gain based on snow mana spent
//! not expressible. Approximated as TutorToHand (permanent, no filter
//! constraint).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Search for Glory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a snow permanent card, a legendary card, or a Saga card, reveal it, put it into your hand, then shuffle. You gain 1 life for each {S} spent to cast this spell.".into(),
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
    // GAP: snow/legendary/Saga filter not expressible
    // GAP: life gain per snow mana spent not expressible
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::permanent(),
        reveal: true,
    }]
}
