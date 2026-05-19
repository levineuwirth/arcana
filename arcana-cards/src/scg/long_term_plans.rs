//! Long-Term Plans — `{2}{U}` instant. "Search your library for a card,
//! then shuffle and put that card third from the top."
//!
//! # GAP: "put the found card third from the top of library" placement is
//! not expressible — TutorToHand puts the card into hand. The tutor into
//! hand is emitted as the closest available approximation; the specific
//! library placement is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Long-Term Plans");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a card, then shuffle and put that card third from the top.".into(),
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
    // GAP: place searched card third from top of library (only TutorToHand available)
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new(),
        reveal: false,
    }]
}
