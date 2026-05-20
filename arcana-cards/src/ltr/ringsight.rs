//! Ringsight — `{1}{U}{B}` sorcery. "The Ring tempts you. Search your
//! library for a card that shares a color with a legendary creature
//! you control, reveal it, put it into your hand, then shuffle."
//!
//! GAP: 'The Ring tempts you' and 'shares a color with a legendary
//! creature you control' aren't expressible. We tutor an unfiltered
//! card from library as the closest approximation, with GAP noted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ringsight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "The Ring tempts you. Search your library for a card that shares a color with a legendary creature you control, reveal it, put it into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: 'Ring tempts you' and 'shares a color with a legendary
    // creature you control' filter — emit an unfiltered tutor.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new(),
        reveal: true,
    }]
}
