//! Final Parting — `{3}{B}{B}` sorcery. "Search your library for two
//! cards. Put one into your hand and the other into your graveyard.
//! Then shuffle."
//!
//! Modeled as a single library tutor to hand. GAP: 'put one into your
//! graveyard' has no library-to-graveyard tutor primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Final Parting");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for two cards. Put one into your hand and the other into your graveyard. Then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: second card going to graveyard has no library-tutor-to-graveyard primitive.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new(),
        reveal: false,
    }]
}
